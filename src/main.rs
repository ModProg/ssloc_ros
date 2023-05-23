use std::io::Cursor;
use std::thread;
use std::time::Duration;

use crossbeam::channel::{bounded, TryRecvError};
use extend::ext;
use image::ImageOutputFormat;
use lib::{for_format, Audio, AudioRecorder, Format};
use nalgebra::UnitQuaternion;
use rosrust::error::ResultExt;
use rosrust::{ros_err, ros_info, ros_warn, Message, Publisher};

mod msgs {
    pub use rosrust_msg::geometry_msgs::*;
                    #[cfg(feature = "odas_messages")]
    pub use rosrust_msg::odas_ros::*;
    pub use rosrust_msg::sensor_msgs::*;
    pub use rosrust_msg::ssloc::*;
    pub use rosrust_msg::std_msgs::{ColorRGBA, Header};
    pub use rosrust_msg::visualization_msgs::*;
}

type Result<T = (), E = rosrust::error::Error> = std::result::Result<T, E>;

mod config;
use config::Config;
use wav::BitDepth;

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

    let mut config_server = rosrust_dynamic_reconfigure::Server::<Config>::new(Config::init()?)?;

    let updating_config = config_server.get_config_updating();

    // TODO consider multiple consumers
    let (audio_channel_send, audio_channel_recv) = bounded(1);
    let audio_recorder = {
        let audio_channel_recv = audio_channel_recv.clone();
        let updating_config = updating_config.clone();
        let frame_id = frame_id.clone();
        thread::Builder::new()
            .name("audio recorder".to_owned())
            .spawn(move || -> Result {
                let audio_topic = rosrust::publish::<msgs::Audio>("~source_audio", 20)?;
                let mut config = updating_config.copy();
                'recorder: while rosrust::is_ok() {
                    if config.use_audio_messages {
                        let audio_channel_send = audio_channel_send.clone();
                        let audio_channel_recv = audio_channel_recv.clone();
                        log_error!(
                            rosrust::subscribe(
                                &config.audio_message_topic,
                                20,
                                move |msg: msgs::Audio| {
                                    if audio_channel_send.is_full() {
                                        match audio_channel_recv.try_recv() {
                                            Ok((stamp, _)) => {
                                                ros_warn!(
                                                    "recording from {stamp} was dropped, ssloc \
                                                    operation too slow"
                                                );
                                            }
                                            Err(TryRecvError::Empty) => { /* was emptied by consumer */
                                            }
                                            Err(TryRecvError::Disconnected) => {
                                                ros_err!(
                                                    "channel disconnected, process must have exited"
                                                );
                                                return;
                                            }
                                        }
                                    }
                                    match audio_channel_send.try_send((msg.header.stamp, Audio::from_wav(Cursor::new(msg.data)))) {
                                        Ok(_) => {}
                                        Err(_) => {
                                            ros_err!("channel disconnected, process must have exited");
                                        }
                                    }
                                },
                            ),
                            "error subscribing to `audio_message_topic` {} {err}",
                            config.audio_message_topic
                        );
                        while rosrust::is_ok() {
                            let update = updating_config.read();
                            if !update.use_audio_messages
                                || config.audio_message_topic != update.audio_message_topic
                            {
                                config = update.clone();
                                continue 'recorder;
                            }
                        }
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
                                if let Err(err) = audio_topic.send(msgs::Audio {
                                    header,
                                    data: audio.wav(BitDepth::ThirtyTwoFloat),
                                }) {
                                    ros_err!("error sending audio message {err}");
                                };
                                if audio_channel_send.is_full() {
                                    match audio_channel_recv.try_recv() {
                                        Ok((stamp, _)) => {
                                            ros_warn!(
                                                "recording from {stamp} was dropped, ssloc \
                                                 operation too slow"
                                            );
                                        }
                                        Err(TryRecvError::Empty) => { /* was emptied by consumer */
                                        }
                                        Err(TryRecvError::Disconnected) => {
                                            ros_err!(
                                                "channel disconnected, process must have exited"
                                            );
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
            })
            .expect("spawning audio thread should not panic")
    };

    let ssloc = thread::Builder::new()
        .name("ssloc".to_owned())
        .spawn(move || -> Result {
            let arrow_markers = rosrust::publish::<msgs::Marker>("~arrow_markers", 20)?;
            let unit_sphere_sst = rosrust::publish::<msgs::UnitSstArray>("~unit_sphere_sst", 20)?;
            let unit_sphere_sst_poses =
                rosrust::publish::<msgs::PoseArray>("~unit_sphere_sst_poses", 20)?;
            let unit_sphere_ssl = rosrust::publish::<msgs::UnitSslArray>("~unit_sphere_ssl", 20)?;
            let unit_sphere_ssl_points =
                rosrust::publish::<msgs::PointCloud2>("~unit_sphere_ssl_points", 20)?;
            let spectrums = rosrust::publish::<msgs::CompressedImage>("~spectrum/compressed", 20)?;
            #[cfg(feature = "odas_messages")]
            let odas_unit_sphere_sst = rosrust::publish::<msgs::OdasSst>("~odas/sst", 10)?;
            #[cfg(feature = "odas_messages")]
            let odas_unit_sphere_sst_poses =
                odas_rosrust::publish::<msgs::PoseArray>("~odas/sst_poses", 10)?;
            #[cfg(feature = "odas_messages")]
            let odas_unit_sphere_ssl = rosrust::publish::<msgs::OdasSslArray>("~odas/ssl", 10)?;
            #[cfg(feature = "odas_messages")]
            let odas_unit_sphere_ssl_points =
                rosrust::publish::<msgs::PointCloud2>("~odas/ssl_pcl2", 10)?;

            let mut config = updating_config.copy();

            'mbss: while rosrust::is_ok() {
                let mbss = config
                    .mbss
                    .create(config.mics[..config.channels as usize].to_owned());
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
                    let Ok((stamp, audio)) = audio_channel_recv.recv() else {
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
                    let spectrum = mbss.analyze_spectrum(&audio);
                    if spectrums.subscriber_count() > 0 {
                        let mut data: Vec<u8> = Vec::new();
                        lib::spec_to_image(spectrum.view())
                            .write_to(&mut Cursor::new(&mut data), ImageOutputFormat::Png)
                            .unwrap();
                        if let Err(e) = spectrums.send(msgs::CompressedImage {
                            header: header.clone(),
                            format: "png".to_string(),
                            data,
                        }) {
                            ros_err!("error sending spectrum image {e}");
                        }
                    }
                    if unit_sphere_ssl.has_subscribers() || unit_sphere_ssl_points.has_subscribers()
                    {
                        let locations =
                            mbss.unit_sphere_spectrum(spectrum.view(), config.mbss_ssl_threashold);

                        if unit_sphere_ssl.has_subscribers() {
                            if let Err(e) = unit_sphere_ssl.send(msgs::UnitSslArray {
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
                            }) {
                                ros_err!("error sending unit sphere ssl {e}");
                            }
                        }
                        if unit_sphere_ssl_points.has_subscribers() {
                            if let Err(e) = unit_sphere_ssl_points.send(msgs::PointCloud2 {
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
                            }) {
                                ros_err!("error sending unit sphere ssl {e}");
                            }
                        }
                    }

                    #[allow(unused_mut)]
                    let mut sst_subbed = arrow_markers.has_subscribers()
                        || unit_sphere_sst.has_subscribers()
                        || unit_sphere_sst_poses.has_subscribers();
                    // #[cfg(feature = "odas_messages")]
                    // sst_subbed |= odas_unit_sphere_sst.has_subscribers()
                    //     || odas_unit_sphere_ssl_points.has_subscribers();
                    if sst_subbed {
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
                                            let position = lib::angles_to_unit_vec(*az, *el);
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
                                odas_unit_sphere_sst.send(msgs::OdasSstArray {
                                    header: header.clone(),
                                    sources: sources
                                        .iter()
                                        .enumerate()
                                        .map(|(id, (az, el, _))| {
                                            let position = lib::angles_to_unit_vec(*az, *el);
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
                        #[allow(unused_mut)]
                        let mut sst_poses_subbed = unit_sphere_sst_poses.has_subscribers();
                        // #[cfg(feature = "odas_messages")]
                        // sst_poses_subbed |= odas_unit_sphere_sst_poses.has_subscribers();
                        if sst_poses_subbed {
                            let poses = msgs::PoseArray {
                                header: header.clone(),
                                poses: sources
                                    .iter()
                                    .map(|(az, el, _)| {
                                        let quaternion = lib::angles_to_quaternion(*az, *el).coords;
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
                                let rotation =
                                    UnitQuaternion::from_euler_angles(0., -el, az).coords;
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
                                            // x: (strength / 2000.).clamp(0.2, 2.),
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
        })
        .expect("should be able to start ssloc process");

    // Create object that maintains 10Hz between sleep requests
    let rate = rosrust::rate(10.0);

    // Breaks when a shutdown signal is sent
    while rosrust::is_ok() {
        rate.sleep();
    }
    ssloc.join().expect("ssloc thread should not panic")?;
    audio_recorder
        .join()
        .expect("audio_recorder should not panic")?;
    Ok(())
}
