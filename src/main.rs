use std::io::Cursor;
use std::thread;
use std::time::Duration;

use crossbeam::channel::{bounded, Receiver, Sender, TryRecvError};
use extend::ext;
use image::ImageOutputFormat;
use nalgebra::UnitQuaternion;
use rosrust::error::ResultExt;
use rosrust::{ros_err, ros_info, ros_warn, Message, Publisher, Time};
use rosrust_dynamic_reconfigure::Updating;
use ssloc::{for_format, Audio, AudioRecorder, Format};

mod msgs {
    #[cfg(all(feature = "odas_messages", feature = "audio_common_msgs_stamped"))]
    rosrust::rosmsg_include! {
        audio_common_msgs/AudioData, audio_common_msgs/AudioDataStamped, audio_common_msgs/AudioInfo,
        geometry_msgs/Point, geometry_msgs/Pose, geometry_msgs/PoseArray, geometry_msgs/Quaternion, geometry_msgs/Vector3,
        odas_ros/OdasSsl, odas_ros/OdasSslArrayStamped, odas_ros/OdasSst, odas_ros/OdasSstArrayStamped,
        sensor_msgs/CompressedImage, sensor_msgs/PointCloud2, sensor_msgs/PointField,
        ssloc_ros_msgs/UnitSsl, ssloc_ros_msgs/UnitSslArray, ssloc_ros_msgs/UnitSst, ssloc_ros_msgs/UnitSstArray,
        std_msgs/ColorRGBA, std_msgs/Header,
        visualization_msgs/Marker,
    }
    #[cfg(all(feature = "odas_messages", not(feature = "audio_common_msgs_stamped")))]
    rosrust::rosmsg_include! {
        audio_common_msgs/AudioData, audio_common_msgs/AudioInfo,
        geometry_msgs/Point, geometry_msgs/Pose, geometry_msgs/PoseArray, geometry_msgs/Quaternion, geometry_msgs/Vector3,
        odas_ros/OdasSsl, odas_ros/OdasSslArrayStamped, odas_ros/OdasSst, odas_ros/OdasSstArrayStamped,
        sensor_msgs/CompressedImage, sensor_msgs/PointCloud2, sensor_msgs/PointField,
        ssloc_ros_msgs/UnitSsl, ssloc_ros_msgs/UnitSslArray, ssloc_ros_msgs/UnitSst, ssloc_ros_msgs/UnitSstArray,
        std_msgs/ColorRGBA, std_msgs/Header,
        visualization_msgs/Marker,
    }
    #[cfg(not(any(feature = "odas_messages", feature = "audio_common_msgs_stamped")))]
    rosrust::rosmsg_include! {
        audio_common_msgs/AudioData, audio_common_msgs/AudioInfo,
        geometry_msgs/Point, geometry_msgs/Pose, geometry_msgs/PoseArray, geometry_msgs/Quaternion, geometry_msgs/Vector3,
        sensor_msgs/CompressedImage, sensor_msgs/PointCloud2, sensor_msgs/PointField,
        ssloc_ros_msgs/UnitSsl, ssloc_ros_msgs/UnitSslArray, ssloc_ros_msgs/UnitSst, ssloc_ros_msgs/UnitSstArray,
        std_msgs/ColorRGBA, std_msgs/Header,
        visualization_msgs/Marker,
    }
    #[cfg(all(not(feature = "odas_messages"), feature = "audio_common_msgs_stamped"))]
    rosrust::rosmsg_include! {
        audio_common_msgs/AudioData, audio_common_msgs/AudioDataStamped, audio_common_msgs/AudioInfo,
        geometry_msgs/Point, geometry_msgs/Pose, geometry_msgs/PoseArray, geometry_msgs/Quaternion, geometry_msgs/Vector3,
        sensor_msgs/CompressedImage, sensor_msgs/PointCloud2, sensor_msgs/PointField,
        ssloc_ros_msgs/UnitSsl, ssloc_ros_msgs/UnitSslArray, ssloc_ros_msgs/UnitSst, ssloc_ros_msgs/UnitSstArray,
        std_msgs/ColorRGBA, std_msgs/Header,
        visualization_msgs/Marker,
    }
    #[cfg(feature = "audio_common_msgs_stamped")]
    pub use audio_common_msgs::AudioDataStamped;
    pub use audio_common_msgs::{AudioData, AudioInfo};
    pub use geometry_msgs::{Point, Pose, PoseArray, Quaternion, Vector3};
    #[cfg(feature = "odas_messages")]
    pub use odas_ros::{OdasSsl, OdasSslArrayStamped, OdasSst, OdasSstArrayStamped};
    pub use sensor_msgs::{CompressedImage, PointCloud2, PointField};
    pub use ssloc_ros_msgs::{UnitSsl, UnitSslArray, UnitSst, UnitSstArray};
    pub use std_msgs::{ColorRGBA, Header};
    pub use visualization_msgs::Marker;
}

type Result<T = (), E = rosrust::error::Error> = std::result::Result<T, E>;

mod config;
use config::Config;

#[ext]
impl<T: Message> Publisher<T> {
    fn has_subscribers(&self) -> bool {
        self.subscriber_count() > 0
    }
}

#[ext]
impl<T, E> Result<T, E> {
    fn log_error(self) {}
}

macro_rules! log_error {
    ($expr:expr, $($fmt:tt)*) => {
        if let Err(err) = $expr {
            ros_err!($($fmt)*, err=err);
        }
    };
}

fn main() -> Result {
    env_logger::init();

    rosrust::init("ssloc");
    let frame_id: String = rosrust::param("~frame_id")
        .expect("should get parameter")
        .get()
        .chain_err(|| "getting ~frame_id parameter")?;
    let ssloc_threads = rosrust::param("~ssloc_threads")
        .expect("should get parameter")
        .get::<usize>()
        .unwrap_or_default()
        .max(1)
        .min(5);

    let mut config_server = rosrust_dynamic_reconfigure::Server::<Config>::new(Config::init()?)?;

    let updating_config = config_server.get_config_updating();

    let (audio_channel_send, audio_channel_recv) = bounded(ssloc_threads);
    let audio_recorder = thread::Builder::new()
        .name("audio recorder".to_owned())
        .spawn(recorder(
            updating_config.clone(),
            frame_id.clone(),
            audio_channel_send,
            audio_channel_recv.clone(),
        ))
        .expect("spawning audio thread should not panic");

    let ssloc: Vec<_> = (0..ssloc_threads)
        .map(|idx| {
            thread::Builder::new()
                .name(format!("ssloc{idx}"))
                .spawn(ssloc(
                    updating_config.clone(),
                    frame_id.clone(),
                    audio_channel_recv.clone(),
                ))
                .expect("should be able to start ssloc process")
        })
        .collect();

    // Create object that maintains 10Hz between sleep requests
    let rate = rosrust::rate(10.0);

    // Breaks when a shutdown signal is sent
    while rosrust::is_ok() {
        rate.sleep();
    }
    for ssloc in ssloc {
        ssloc.join().expect("ssloc thread should not panic")?;
    }
    audio_recorder
        .join()
        .expect("audio_recorder should not panic")?;

    Ok(())
}

fn recorder(
    updating_config: Updating<Config>,
    #[cfg_attr(not(eature = "audio_common_msgs_stamped"), allow(unused))]
    frame_id: String,
    audio_channel_send: Sender<(Time, Audio)>,
    audio_channel_recv: Receiver<(Time, Audio)>,
) -> impl FnOnce() -> Result {
    move || {
        let audio_topic = rosrust::publish::<msgs::AudioData>("~audio", 10)?;
        #[cfg(feature = "audio_common_msgs_stamped")]
        let audio_stamped_topic = rosrust::publish::<msgs::AudioDataStamped>("~audio_stamped", 10)?;
        let mut audio_info_topic = rosrust::publish::<msgs::AudioInfo>("~audio_info", 1)?;
        audio_info_topic.set_latching(true);

        let mut config = updating_config.copy();
        'recorder: while rosrust::is_ok() {
            if config.use_audio_messages {
                todo!("audio messages")
                // let audio_channel_send = audio_channel_send.clone();
                // let audio_channel_recv = audio_channel_recv.clone();
                // log_error!(
                //     rosrust::subscribe(
                //         &config.audio_message_topic,
                //         20,
                //         move |msg: msgs::Audio| {
                //             if audio_channel_send.is_full() {
                //                 match audio_channel_recv.try_recv() {
                //                     Ok((stamp, _)) => {
                //                         ros_warn!(
                //                             "recording from {stamp}
                // was dropped, ssloc \
                //                             operation too slow"
                //                         );
                //                     }
                //                     Err(TryRecvError::Empty) => { /*
                // was emptied by consumer */
                //                     }
                //                     Err(TryRecvError::Disconnected)
                // => {
                // ros_err!(
                // "channel disconnected, process must have exited"
                //                         );
                //                         return;
                //                     }
                //                 }
                //             }
                //             match
                // audio_channel_send.send((msg.header.stamp,
                // Audio::from_wav(Cursor::new(msg.data)))) {
                //                 Ok(_) => {}
                //                 Err(_) => {
                //                     ros_err!("channel disconnected,
                // process must have exited");
                //                 }
                //             }
                //         },
                //     ),
                //     "error subscribing to `audio_message_topic` {}
                // {err}",     config.
                // audio_message_topic );
                // while rosrust::is_ok() {
                //     let update = updating_config.read();
                //     if !update.use_audio_messages
                //         || config.audio_message_topic !=
                // update.audio_message_topic
                //     {
                //         config = update.clone();
                //         continue 'recorder;
                //     }
                // }
            } else {
                for_format!(config.format, {
                    let mut recorder = match AudioRecorder::<FORMAT>::new(
                        config.device.name.clone(),
                        config.channels.into(),
                        config.rate.into(),
                        config.format,
                        config.localisation_frame,
                    ) {
                        Ok(recorder) => recorder,
                        Err(e) => {
                            ros_err!("error creating the audio recorder {e}");
                            thread::sleep(Duration::from_secs(1));
                            continue;
                        }
                    };

                    while rosrust::is_ok() {
                        let stamp = rosrust::now();
                        #[cfg(feature = "audio_common_msgs_stamped")]
                        let header = msgs::Header {
                            stamp,
                            frame_id: frame_id.clone(),
                            ..Default::default()
                        };
                        {
                            let update = updating_config.read();
                            if update.channels != config.channels
                                || update.device != config.device
                                || update.rate != config.rate
                                || update.format != config.format
                                || update.localisation_frame != config.localisation_frame
                                || update.use_audio_messages
                            {
                                config = update.clone();
                                continue 'recorder;
                            }
                        }
                        let audio = match recorder.record() {
                            Ok(audio) => audio,
                            Err(err) => {
                                ros_err!("error recording audio {err}");
                                continue 'recorder;
                            }
                        };
                        let subbed = audio_topic.has_subscribers();
                        #[cfg(feature = "audio_common_msgs_stamped")]
                        let subbed = audio_stamped_topic.has_subscribers();
                        if subbed {
                            // TODO consider supporting more than one output format.
                            let audio = msgs::AudioData {
                                data: audio.to_interleaved().flat_map(f32::to_le_bytes).collect(),
                            };
                            #[cfg(feature = "audio_common_msgs_stamped")]
                            log_error!(
                                audio_stamped_topic.send(msgs::AudioDataStamped {
                                    header: header.clone(),
                                    audio: audio.clone()
                                }),
                                "error sending audio message {err}"
                            );
                            log_error!(
                                audio_topic.send(audio),
                                "error sending audio message {err}"
                            );
                        }
                        if audio_channel_send.is_full() {
                            match audio_channel_recv.try_recv() {
                                Ok((stamp, _)) => {
                                    ros_warn!(
                                        "recording from {stamp} was dropped, ssloc operation too \
                                         slow"
                                    );
                                }
                                Err(TryRecvError::Empty) => { /* was emptied by consumer */ }
                                Err(TryRecvError::Disconnected) => {
                                    ros_err!("channel disconnected, process must have exited");
                                    return Ok(());
                                }
                            }
                        }
                        match audio_channel_send.try_send((stamp, audio)) {
                            Ok(_) => {}
                            Err(_) => {
                                ros_err!("channel disconnected, process must have exited");
                                return Ok(());
                            }
                        }
                    }
                });
            }
        }
        Ok(())
    }
}

fn ssloc(
    updating_config: Updating<Config>,
    frame_id: String,
    audio_channel_recv: Receiver<(Time, Audio)>,
) -> impl FnOnce() -> Result {
    move || {
        let arrow_markers = rosrust::publish::<msgs::Marker>("~arrow_markers", 20)?;
        let unit_sphere_sst = rosrust::publish::<msgs::UnitSstArray>("~unit_sphere_sst", 20)?;
        let unit_sphere_sst_poses =
            rosrust::publish::<msgs::PoseArray>("~unit_sphere_sst_poses", 20)?;
        let unit_sphere_ssl = rosrust::publish::<msgs::UnitSslArray>("~unit_sphere_ssl", 20)?;
        let unit_sphere_ssl_points =
            rosrust::publish::<msgs::PointCloud2>("~unit_sphere_ssl_points", 20)?;
        let spectrums = rosrust::publish::<msgs::CompressedImage>("~spectrum/compressed", 20)?;

        #[cfg(feature = "odas_messages")]
        let odas_unit_sphere_sst = rosrust::publish::<msgs::OdasSstArrayStamped>("~odas/sst", 10)?;
        #[cfg(feature = "odas_messages")]
        let odas_unit_sphere_sst_poses =
            rosrust::publish::<msgs::PoseArray>("~odas/sst_poses", 10)?;
        #[cfg(feature = "odas_messages")]
        let odas_unit_sphere_ssl = rosrust::publish::<msgs::OdasSslArrayStamped>("~odas/ssl", 10)?;
        #[cfg(feature = "odas_messages")]
        let odas_unit_sphere_ssl_points =
            rosrust::publish::<msgs::PointCloud2>("~odas/ssl_pcl2", 10)?;

        let mut config = updating_config.copy();

        'mbss: while rosrust::is_ok() {
            let mbss = config.mbss.create(
                config.mics[..config.channels as usize]
                    .iter()
                    .filter(|(_, enabled)| *enabled)
                    .map(|(pos, _)| pos.clone()),
            );
            while rosrust::is_ok() {
                let max_sources = {
                    let update = updating_config.read();
                    if update.channels != config.channels
                        || update.mics != config.mics
                        || update.mbss != config.mbss
                    {
                        config = update.clone();
                        continue 'mbss;
                    }
                    update.max_sources.into()
                };
                let Ok((stamp, mut audio)) = audio_channel_recv.recv() else {
                    ros_err!("channel disconnected, process must have exited");
                    return Ok(());
                };
                let header = msgs::Header {
                    stamp,
                    frame_id: frame_id.clone(),
                    ..Default::default()
                };
                if audio.channels() != config.channels as usize {
                    ros_info!("channels of recording missmatched, probably config was updated");
                    continue;
                }
                audio.retain_channels(|c| config.mics[c].1);
                let spectrum = mbss.analyze_spectrum(&audio);
                if spectrums.subscriber_count() > 0 {
                    let mut data: Vec<u8> = Vec::new();
                    ssloc::spec_to_image(spectrum.view())
                        .write_to(&mut Cursor::new(&mut data), ImageOutputFormat::Png)
                        .unwrap();
                    log_error!(
                        spectrums.send(msgs::CompressedImage {
                            header: header.clone(),
                            format: "png".to_string(),
                            data,
                        }),
                        "error sending spectrum image {err}"
                    );
                }

                let subbed =
                    unit_sphere_ssl.has_subscribers() || unit_sphere_ssl_points.has_subscribers();
                #[cfg(feature = "odas_messages")]
                let subbed = subbed
                    || odas_unit_sphere_ssl.has_subscribers()
                    || odas_unit_sphere_ssl_points.has_subscribers();
                if subbed {
                    let locations =
                        mbss.unit_sphere_spectrum(spectrum.view(), config.mbss_ssl_threashold);

                    if unit_sphere_ssl.has_subscribers() {
                        log_error!(
                            unit_sphere_ssl.send(msgs::UnitSslArray {
                                header: header.clone(),
                                sources: locations
                                    .iter()
                                    .map(|(position, e)| msgs::UnitSsl {
                                        x: position.x,
                                        y: position.y,
                                        z: position.z,
                                        E: *e,
                                    })
                                    .collect(),
                            }),
                            "error sending unit sphere ssl {err}"
                        );
                    }
                    if unit_sphere_ssl.has_subscribers() {
                        #[cfg(feature = "odas_messages")]
                        log_error!(
                            odas_unit_sphere_ssl.send(msgs::OdasSslArrayStamped {
                                header: header.clone(),
                                sources: locations
                                    .iter()
                                    .map(|(position, e)| msgs::OdasSsl {
                                        x: position.x,
                                        y: position.y,
                                        z: position.z,
                                        E: *e,
                                    })
                                    .collect(),
                            }),
                            "error sending unit sphere ssl {err}"
                        );
                    }

                    if unit_sphere_ssl_points.has_subscribers() {
                        let msg = msgs::PointCloud2 {
                            header: header.clone(),
                            fields: vec![
                                msgs::PointField {
                                    name: "x".to_owned(),
                                    offset: 0,
                                    datatype: msgs::PointField::FLOAT32,
                                    count: 1,
                                },
                                msgs::PointField {
                                    name: "y".to_owned(),
                                    offset: 4,
                                    datatype: msgs::PointField::FLOAT32,
                                    count: 1,
                                },
                                msgs::PointField {
                                    name: "z".to_owned(),
                                    offset: 8,
                                    datatype: msgs::PointField::FLOAT32,
                                    count: 1,
                                },
                                msgs::PointField {
                                    name: "intensity".to_owned(),
                                    offset: 12,
                                    datatype: msgs::PointField::FLOAT32,
                                    count: 1,
                                },
                            ],
                            is_bigendian: false,
                            point_step: 16,
                            data: locations
                                .iter()
                                .flat_map(|(position, e)| {
                                    [position.x, position.y, position.z, *e]
                                        .into_iter()
                                        .flat_map(|c| (c as f32).to_le_bytes())
                                })
                                .collect(),
                            height: 1,
                            width: locations.len() as u32,
                            row_step: locations.len() as u32,
                            is_dense: true,
                        };
                        #[cfg(feature = "odas_messages")]
                        log_error!(
                            odas_unit_sphere_ssl_points.send(msg.clone()),
                            "error sending unit sphere ssl {err}"
                        );
                        log_error!(
                            unit_sphere_ssl_points.send(msg),
                            "error sending unit sphere ssl {err}"
                        );
                    }
                }

                let subbed = arrow_markers.has_subscribers()
                    || unit_sphere_sst.has_subscribers()
                    || unit_sphere_sst_poses.has_subscribers();
                #[cfg(feature = "odas_messages")]
                let subbed = subbed
                    || odas_unit_sphere_sst.has_subscribers()
                    || odas_unit_sphere_ssl_points.has_subscribers();

                if subbed {
                    let sources: Vec<_> = mbss
                        .find_sources(spectrum.view(), max_sources)
                        .into_iter()
                        .filter(|(.., strength)| *strength > config.mbss_ssl_threashold)
                        .collect();

                    if unit_sphere_sst.has_subscribers() {
                        log_error!(
                            unit_sphere_sst.send(msgs::UnitSstArray {
                                header: header.clone(),
                                sources: sources
                                    .iter()
                                    .enumerate()
                                    .map(|(id, (az, el, _))| {
                                        let position = ssloc::angles_to_unit_vec(*az, *el);
                                        msgs::UnitSst {
                                            id: id as i64,
                                            activity: 0.,
                                            x: position.x,
                                            y: position.y,
                                            z: position.z,
                                        }
                                    })
                                    .collect(),
                            }),
                            "error sending the unit sphere sst message: {err}"
                        );
                    }
                    #[cfg(feature = "odas_messages")]
                    if odas_unit_sphere_sst.has_subscribers() {
                        log_error!(
                            odas_unit_sphere_sst.send(msgs::OdasSstArrayStamped {
                                header: header.clone(),
                                sources: sources
                                    .iter()
                                    .enumerate()
                                    .map(|(id, (az, el, _))| {
                                        let position = ssloc::angles_to_unit_vec(*az, *el);
                                        msgs::OdasSst {
                                            id: id as i64,
                                            activity: 0.,
                                            x: position.x,
                                            y: position.y,
                                            z: position.z,
                                        }
                                    })
                                    .collect(),
                            }),
                            "error sending the unit sphere sst message: {err}"
                        );
                    }

                    let sst_poses_subbed = unit_sphere_sst_poses.has_subscribers();
                    #[cfg(feature = "odas_messages")]
                    let sst_poses_subbed =
                        sst_poses_subbed || odas_unit_sphere_sst_poses.has_subscribers();

                    if sst_poses_subbed {
                        let poses = msgs::PoseArray {
                            header: header.clone(),
                            poses: sources
                                .iter()
                                .map(|(az, el, _)| {
                                    let quaternion = ssloc::angles_to_quaternion(*az, *el).coords;
                                    msgs::Pose {
                                        orientation: msgs::Quaternion {
                                            x: quaternion.x,
                                            y: quaternion.y,
                                            z: quaternion.z,
                                            w: quaternion.w,
                                        },
                                        ..Default::default()
                                    }
                                })
                                .collect(),
                        };
                        #[cfg(feature = "odas_messages")]
                        if odas_unit_sphere_sst_poses.has_subscribers() {
                            log_error!(
                                odas_unit_sphere_sst_poses.send(poses.clone()),
                                "error sending the odas unit sphere sst message: {err}"
                            );
                        }
                        if unit_sphere_sst_poses.has_subscribers() {
                            log_error!(
                                unit_sphere_sst_poses.send(poses),
                                "error sending the unit sphere sst message: {err}"
                            );
                        }
                    }
                    if arrow_markers.has_subscribers() {
                        for (idx, (az, el, _strength)) in sources.into_iter().enumerate() {
                            let rotation = UnitQuaternion::from_euler_angles(0., -el, az).coords;
                            log_error!(
                                arrow_markers.send(msgs::Marker {
                                    header: header.clone(),
                                    ns: "sslocate".to_string(),
                                    id: idx as i32 + 1,
                                    type_: msgs::Marker::ARROW as i32,
                                    pose: msgs::Pose {
                                        position: msgs::Point {
                                            x: 0.,
                                            y: 0.,
                                            z: 0.,
                                        },
                                        orientation: msgs::Quaternion {
                                            x: rotation.x,
                                            y: rotation.y,
                                            z: rotation.z,
                                            w: rotation.w,
                                        },
                                    },
                                    color: msgs::ColorRGBA {
                                        r: 1.,
                                        a: 1.,
                                        ..Default::default()
                                    },
                                    scale: msgs::Vector3 {
                                        x: 1.,
                                        y: 0.2,
                                        z: 0.2,
                                    },
                                    action: msgs::Marker::ADD as i32,
                                    lifetime: rosrust::Duration::from_seconds(1),
                                    ..Default::default()
                                }),
                                "error sending marker {err}"
                            );
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
