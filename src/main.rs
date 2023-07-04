use std::io::Cursor;
use std::mem::size_of;
use std::sync::atomic::AtomicI64;
use std::sync::{atomic, Arc};
use std::time::Duration;
use std::{iter, mem, thread};

use crossbeam::channel::{bounded, Receiver, Sender, TryRecvError};
use extend::ext;
use image::ImageOutputFormat;
use itertools::Itertools;
use parking_lot::Mutex;
use rosrust::error::ResultExt;
use rosrust::{ros_err, ros_info, ros_warn, ros_warn_throttle, Message, Publisher, Time};
use rosrust_dynamic_reconfigure::Updating;
use ssloc::mbss::angular_distance;
use ssloc::{for_format, Audio, AudioRecorder, DelayAndSum, Direction, Format, PcmFormat, F};

#[cfg(feature = "builtin-msgs")]
mod msgs;

#[cfg(not(feature = "builtin-msgs"))]
mod msgs {
    #[cfg(all(feature = "odas-msgs", feature = "audio_common_msgs-stamped"))]
    rosrust::rosmsg_include! {
        audio_common_msgs/AudioData, audio_common_msgs/AudioDataStamped, audio_common_msgs/AudioInfo,
        geometry_msgs/Point, geometry_msgs/Pose, geometry_msgs/PoseArray, geometry_msgs/Quaternion, geometry_msgs/Vector3,
        odas_ros/OdasSsl, odas_ros/OdasSslArrayStamped, odas_ros/OdasSst, odas_ros/OdasSstArrayStamped,
        sensor_msgs/CompressedImage, sensor_msgs/PointCloud2, sensor_msgs/PointField,
        ssloc_ros_msgs/Ssl, ssloc_ros_msgs/SslArray, ssloc_ros_msgs/Sst, ssloc_ros_msgs/SstArray, ssloc_ros_msgs/SssMapping,
        std_msgs/ColorRGBA, std_msgs/Header,
        visualization_msgs/Marker,
    }
    #[cfg(all(feature = "odas-msgs", not(feature = "audio_common_msgs-stamped")))]
    rosrust::rosmsg_include! {
        audio_common_msgs/AudioData, audio_common_msgs/AudioInfo,
        geometry_msgs/Point, geometry_msgs/Pose, geometry_msgs/PoseArray, geometry_msgs/Quaternion, geometry_msgs/Vector3,
        odas_ros/OdasSsl, odas_ros/OdasSslArrayStamped, odas_ros/OdasSst, odas_ros/OdasSstArrayStamped,
        sensor_msgs/CompressedImage, sensor_msgs/PointCloud2, sensor_msgs/PointField,
        ssloc_ros_msgs/Ssl, ssloc_ros_msgs/SslArray, ssloc_ros_msgs/Sst, ssloc_ros_msgs/SstArray, ssloc_ros_msgs/SssMapping,
        std_msgs/ColorRGBA, std_msgs/Header,
        visualization_msgs/Marker,
    }
    #[cfg(not(any(feature = "odas-msgs", feature = "audio_common_msgs-stamped")))]
    rosrust::rosmsg_include! {
        audio_common_msgs/AudioData, audio_common_msgs/AudioInfo,
        geometry_msgs/Point, geometry_msgs/Pose, geometry_msgs/PoseArray, geometry_msgs/Quaternion, geometry_msgs/Vector3,
        sensor_msgs/CompressedImage, sensor_msgs/PointCloud2, sensor_msgs/PointField,
        ssloc_ros_msgs/Ssl, ssloc_ros_msgs/SslArray, ssloc_ros_msgs/Sst, ssloc_ros_msgs/SstArray, ssloc_ros_msgs/SssMapping,
        std_msgs/ColorRGBA, std_msgs/Header,
        visualization_msgs/Marker,
    }
    #[cfg(all(not(feature = "odas-msgs"), feature = "audio_common_msgs-stamped"))]
    rosrust::rosmsg_include! {
        audio_common_msgs/AudioData, audio_common_msgs/AudioDataStamped, audio_common_msgs/AudioInfo,
        geometry_msgs/Point, geometry_msgs/Pose, geometry_msgs/PoseArray, geometry_msgs/Quaternion, geometry_msgs/Vector3,
        sensor_msgs/CompressedImage, sensor_msgs/PointCloud2, sensor_msgs/PointField,
        ssloc_ros_msgs/Ssl, ssloc_ros_msgs/SslArray, ssloc_ros_msgs/Sst, ssloc_ros_msgs/SstArray, ssloc_ros_msgs/SssMapping,
        std_msgs/ColorRGBA, std_msgs/Header,
        visualization_msgs/Marker,
    }
    #[cfg(feature = "audio_common_msgs-stamped")]
    pub use audio_common_msgs::AudioDataStamped;
    pub use audio_common_msgs::{AudioData, AudioInfo};
    pub use geometry_msgs::{Point, Pose, PoseArray, Quaternion, Vector3};
    #[cfg(feature = "odas-msgs")]
    pub use odas_ros::{OdasSsl, OdasSslArrayStamped, OdasSst, OdasSstArrayStamped};
    pub use sensor_msgs::{CompressedImage, PointCloud2, PointField};
    pub use ssloc_ros_msgs::{Ssl, SslArray, SssMapping, Sst, SstArray};
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
macro_rules! continue_error {
    ($expr:expr, $($fmt:tt)*) => {
        match $expr {
            Ok(o) => o,
            Err(err) => {
                ros_err!($($fmt)*, err=err);
                continue;
            },
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
    let recording_only: bool = rosrust::param("~recording_only")
        .expect("should get parameter")
        .get()
        .unwrap_or_default();
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
            recording_only,
        ))
        .expect("spawning audio thread should not panic");

    let ssloc: Vec<_> = (0..(!recording_only)
        .then_some(ssloc_threads)
        .unwrap_or_default())
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
    #[cfg_attr(not(feature = "audio_common_msgs-stamped"), allow(unused))] frame_id: String,
    audio_channel_send: Sender<(Time, Audio)>,
    audio_channel_recv: Receiver<(Time, Audio)>,
    recording_only: bool,
) -> impl FnOnce() -> Result {
    move || {
        let audio_topic = rosrust::publish::<msgs::AudioData>("~audio", 10)?;
        #[cfg(feature = "audio_common_msgs-stamped")]
        let audio_stamped_topic = rosrust::publish::<msgs::AudioDataStamped>("~audio_stamped", 10)?;
        let mut audio_info_topic = rosrust::publish::<msgs::AudioInfo>("~audio_info", 1)?;
        audio_info_topic.set_latching(true);

        let mut config = updating_config.copy();
        'recorder: while rosrust::is_ok() {
            if let Some(audio_topic) = config.audio_message_topic.clone() {
                let audio_channel_send = audio_channel_send.clone();
                let audio_channel_recv = audio_channel_recv.clone();
                let audio_info_topic = format!("{audio_topic}/audio_info");
                let audio_stamped_topic = format!("{audio_topic}/audio_stamped");
                let audio_info = Arc::new(Mutex::new(None));
                let recorded_with = Arc::new(Mutex::new(None));
                let _audio_info_subscriber = {
                    let audio_info = audio_info.clone();
                    let audio_info_topic = audio_info_topic.clone();
                    continue_error!(
                        rosrust::subscribe(
                            &audio_info_topic.clone(),
                            1,
                            move |info: msgs::AudioInfo| {
                                ros_warn!("`{audio_info_topic} = {info:?}");
                                if info.coding_format != "wave" {
                                    ros_err!(
                                        "unsuported coding_format: `{}`, only `wave` is supported.",
                                        info.coding_format
                                    );
                                    return;
                                }
                                if info.channels as u16 != config.channels {
                                    todo!("missmatched channel count")
                                }
                                *audio_info.try_lock_for(Duration::from_secs(1)).unwrap() =
                                    Some(info);
                            }
                        ),
                        "{err}"
                    )
                };
                // let timestamp = Arc::new(Mutex::new(rosrust::now()));
                // let audio_data = Arc::new(Mutex::new(Vec::<u8>::new()));
                let _audio_stamped_recorder = {
                    let audio_info = audio_info.clone();
                    let recorded_with = recorded_with.clone();
                    // let updating_config = updating_config.clone();
                    continue_error!(
                        rosrust::subscribe(
                            &audio_stamped_topic,
                            20,
                            move |msg: msgs::AudioDataStamped| {
                                let Some(audio_info) = audio_info
                                        .try_lock_for(Duration::from_secs(1))
                                        .unwrap()
                                        .clone()
                                else {
                                    ros_warn_throttle!(1., "`{audio_info_topic}` not yet recieved");
                                    return;
                                };
                                // let mut audio_data =
                                //     audio_data.try_lock_for(Duration::from_secs(1)).unwrap();
                                let mut recorded_with =
                                    recorded_with.try_lock_for(Duration::from_secs(1)).unwrap();
                                if recorded_with.is_none() {
                                    *recorded_with = Some(audio_info.clone());
                                };
                                // let len = { updating_config.read().localisation_frame };
                                let rec_with = recorded_with.as_ref().unwrap();
                                let sample_format: PcmFormat = match rec_with.sample_format.parse()
                                {
                                    Ok(ok) => ok,
                                    Err(err) => {
                                        ros_err!(
                                            "Unsupported sample_format `{}`: {err:?}",
                                            recorded_with.as_ref().unwrap().sample_format
                                        );
                                        return;
                                    }
                                };
                                // TODO make recording possible using longer time frames than
                                // sender
                                // if /* (&audio_info != rec_with
                                    // || audio_data.len() as F
                                    //     >= rec_with.sample_rate as F
                                    //         * len
                                    //         * rec_with.channels as F
                                    //         * sample_format.bytes() as F)
                                    // &&*/ // !audio_data.is_empty()
                                {
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
                                                    "channel disconnected, process must have \
                                                     exited"
                                                );
                                                return;
                                            }
                                        }
                                    }
                                    let audio = Audio::from_pcm_bytes(
                                        sample_format,
                                        audio_info.sample_rate.into(),
                                        audio_info.channels.into(),
                                        &msg.audio.data,
                                    );
                                    match audio_channel_send.send((msg.header.stamp, audio)) {
                                        Ok(_) => {}
                                        Err(_) => {
                                            ros_err!(
                                                "channel disconnected, process must have exited"
                                            );
                                        }
                                    }
                                    // audio_data.clear();
                                    // *recorded_with = Some(audio_info);
                                    // *timestamp.try_lock_for(Duration::from_secs(1)).unwrap() =
                                    //     msg.header.stamp;
                                }
                                // audio_data.extend_from_slice(&msg.audio.data)
                            },
                        ),
                        "error subscribing to `audio_message_topic` {} {err}",
                        audio_topic
                    )
                };
                let rate = rosrust::rate(10.0);
                while rosrust::is_ok() {
                    rate.sleep();
                    let update = updating_config.read();
                    if config.audio_message_topic != update.audio_message_topic
                        || config.channels != update.channels
                    {
                        config = update.clone();
                        continue 'recorder;
                    }
                }
            } else {
                log_error!(
                    audio_info_topic.send(msgs::AudioInfo {
                        channels: config.channels as u8,
                        sample_rate: config.rate.into(),
                        sample_format: "F32LE".into(),
                        bitrate: (size_of::<f32>() as u32) * 8 * config.rate as u32,
                        coding_format: "wave".into(),
                    }),
                    "error sending audio info message {err}"
                );
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
                        #[cfg(feature = "audio_common_msgs-stamped")]
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
                                || update.audio_message_topic.is_some()
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
                        #[cfg_attr(feature = "audio_common_msgs-stamped", allow(unused))]
                        let subbed = audio_topic.has_subscribers();
                        #[cfg(feature = "audio_common_msgs-stamped")]
                        let subbed = audio_stamped_topic.has_subscribers();
                        if subbed {
                            // TODO consider supporting more than one output format.
                            let msg = msgs::AudioData {
                                data: audio.to_interleaved().flat_map(f32::to_le_bytes).collect(),
                            };
                            #[cfg(feature = "audio_common_msgs-stamped")]
                            log_error!(
                                audio_stamped_topic.send(msgs::AudioDataStamped {
                                    header: header.clone(),
                                    audio: msg.clone()
                                }),
                                "error sending audio message {err}"
                            );
                            log_error!(audio_topic.send(msg), "error sending audio message {err}");
                        }
                        if !recording_only {
                            if audio_channel_send.is_full() {
                                match audio_channel_recv.try_recv() {
                                    Ok((stamp, _)) => {
                                        ros_warn!(
                                            "recording from {stamp} was dropped, ssloc operation \
                                             too slow"
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
                    }
                });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Track {
    direction: Direction,
    stamp: rosrust::Time,
    power: F,
    id: i64,
    channel: Option<usize>,
}

fn ssloc(
    updating_config: Updating<Config>,
    frame_id: String,
    audio_channel_recv: Receiver<(Time, Audio)>,
) -> impl FnOnce() -> Result {
    move || {
        let arrow_markers = rosrust::publish::<msgs::Marker>("~arrow_markers", 20)?;
        let unit_sphere_sst = rosrust::publish::<msgs::SstArray>("~unit_sphere_sst", 20)?;
        let unit_sphere_sst_poses =
            rosrust::publish::<msgs::PoseArray>("~unit_sphere_sst_poses", 20)?;
        let unit_sphere_ssl = rosrust::publish::<msgs::SslArray>("~unit_sphere_ssl", 20)?;
        let unit_sphere_ssl_points =
            rosrust::publish::<msgs::PointCloud2>("~unit_sphere_ssl_points", 20)?;
        let spectrums = rosrust::publish::<msgs::CompressedImage>("~spectrum/compressed", 20)?;
        let mut sss_mapping = rosrust::publish::<msgs::SssMapping>("~sss/mapping", 10)?;
        let sss_audio_topic = rosrust::publish::<msgs::AudioData>("~sss/audio", 10)?;
        #[cfg(feature = "audio_common_msgs-stamped")]
        let sss_audio_stamped_topic =
            rosrust::publish::<msgs::AudioDataStamped>("~sss/audio_stamped", 10)?;
        let mut sss_audio_info_topic = rosrust::publish::<msgs::AudioInfo>("~sss/audio_info", 10)?;
        sss_audio_info_topic.set_latching(true);
        sss_mapping.set_latching(true);

        #[cfg(feature = "odas-msgs")]
        let odas_unit_sphere_sst = rosrust::publish::<msgs::OdasSstArrayStamped>("~odas/sst", 10)?;
        #[cfg(feature = "odas-msgs")]
        let odas_unit_sphere_sst_poses =
            rosrust::publish::<msgs::PoseArray>("~odas/sst_poses", 10)?;
        #[cfg(feature = "odas-msgs")]
        let odas_unit_sphere_ssl = rosrust::publish::<msgs::OdasSslArrayStamped>("~odas/ssl", 10)?;
        #[cfg(feature = "odas-msgs")]
        let odas_unit_sphere_ssl_points =
            rosrust::publish::<msgs::PointCloud2>("~odas/ssl_pcl2", 10)?;

        let mut config = updating_config.copy();

        let last_tracks: Vec<Track> = Vec::new();
        let last_tracks = Arc::new(Mutex::new(last_tracks));

        // continuiously increases and will wrap at some point
        let track_index = AtomicI64::default();

        'mbss: while rosrust::is_ok() {
            let mics = config.mics[..config.channels as usize]
                .iter()
                .filter(|(_, enabled)| *enabled)
                .map(|(pos, _)| *pos)
                .collect_vec();
            let mbss = config.mbss.create(mics.clone());
            let das = DelayAndSum {
                speed_of_sound: config.mbss.speed_of_sound,
                mics,
                ..Default::default()
            };
            while rosrust::is_ok() {
                {
                    let update = updating_config.read();
                    if update.channels != config.channels
                        || update.mics != config.mics
                        || update.mbss != config.mbss
                    {
                        config = update.clone();
                        continue 'mbss;
                    }
                    config.max_sources = update.max_sources;
                    config.tracking_persistence = update.tracking_persistence;
                    config.mbss_ssl_threshold = update.mbss_ssl_threshold;
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
                #[cfg(feature = "odas-msgs")]
                let subbed = subbed
                    || odas_unit_sphere_ssl.has_subscribers()
                    || odas_unit_sphere_ssl_points.has_subscribers();
                if subbed {
                    let locations = mbss.spectrum(spectrum.view(), config.mbss_ssl_threshold);

                    if unit_sphere_ssl.has_subscribers() {
                        log_error!(
                            unit_sphere_ssl.send(msgs::SslArray {
                                header: header.clone(),
                                sources: locations
                                    .iter()
                                    .map(|(direction, p)| {
                                        let position = direction.to_unit_vec();
                                        msgs::Ssl {
                                            x: position.x,
                                            y: position.y,
                                            z: position.z,
                                            azimuth: direction.azimuth,
                                            elevation: direction.elevation,
                                            P: *p,
                                        }
                                    })
                                    .collect(),
                            }),
                            "error sending unit sphere ssl {err}"
                        );
                    }
                    if unit_sphere_ssl.has_subscribers() {
                        #[cfg(feature = "odas-msgs")]
                        log_error!(
                            odas_unit_sphere_ssl.send(msgs::OdasSslArrayStamped {
                                header: header.clone(),
                                sources: locations
                                    .iter()
                                    .map(|(direction, e)| {
                                        let position = direction.to_unit_vec();
                                        msgs::OdasSsl {
                                            x: position.x,
                                            y: position.y,
                                            z: position.z,
                                            E: *e,
                                        }
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
                                .flat_map(|(direction, e)| {
                                    let position = direction.to_unit_vec();
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
                        #[cfg(feature = "odas-msgs")]
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
                #[cfg(feature = "odas-msgs")]
                let subbed = subbed
                    || odas_unit_sphere_sst.has_subscribers()
                    || odas_unit_sphere_ssl_points.has_subscribers();

                let sss_subbed = sss_audio_topic.has_subscribers();
                #[cfg(feature = "audio_common_msgs-stamped")]
                let sss_subbed = sss_subbed || sss_audio_stamped_topic.has_subscribers();

                if subbed || sss_subbed {
                    let mut last_tracks = last_tracks
                        .try_lock_for(Duration::from_secs(1))
                        .expect("should not deadlock");
                    if last_tracks.iter().any(|track| track.stamp > stamp) {
                        ros_info!("skipping publish of out of order poses");
                        continue;
                    }

                    let mut new_tracks = Vec::new();

                    for (direction, intensity) in mbss
                        .find_sources(spectrum.view(), config.max_sources.into())
                        .into_iter()
                        .filter(|(.., strength)| *strength > config.mbss_ssl_threshold)
                    {
                        let neighboors: Vec<_> = last_tracks
                            .iter()
                            .enumerate()
                            .filter(|(_, track)| {
                                angular_distance(track.direction, direction) < config.mbss.min_angle
                            })
                            .collect();
                        match neighboors.as_slice() {
                            [] => new_tracks.push(Track {
                                direction,
                                stamp,
                                power: intensity,
                                id: track_index.fetch_add(1, atomic::Ordering::SeqCst),
                                channel: None,
                            }),
                            multiple => {
                                // first one has highest intensity
                                let neighboor = multiple[0].1;
                                new_tracks.push(Track {
                                    direction,
                                    stamp,
                                    power: (neighboor.power * 0.8).max(intensity),
                                    id: neighboor.id,
                                    channel: neighboor.channel,
                                });
                                let idxs = multiple.iter().rev().map(|(i, _)| *i).collect_vec();
                                for i in idxs {
                                    last_tracks.remove(i);
                                }
                            }
                        }
                    }
                    let mut sources: Vec<_> = new_tracks
                        .into_iter()
                        .chain(
                            mem::take::<Vec<_>>(last_tracks.as_mut())
                                .into_iter()
                                .filter(|track| {
                                    track.stamp.seconds() + config.tracking_persistence
                                        >= stamp.seconds()
                                }),
                        )
                        .collect();
                    sources.sort_unstable_by(|a, b| b.power.total_cmp(&a.power));
                    sources = if sources.len() > config.max_sources.into() {
                        sources[0..config.max_sources.into()].to_vec()
                    } else {
                        sources
                    };
                    // Assign channels to tracks without channels
                    let mut min = 0;
                    let mut sources = sources
                        .iter()
                        .map(|&(mut t)| {
                            if t.channel.is_some() {
                                t
                            } else {
                                min = (min..)
                                    .find(|&c| !sources.iter().any(|t| t.channel == Some(c)))
                                    .expect("there should be a usize that is not taken");
                                t.channel = Some(min);
                                t
                            }
                        })
                        .collect_vec();
                    *last_tracks = sources.clone();
                    drop(last_tracks);
                    if unit_sphere_sst.has_subscribers() {
                        log_error!(
                            unit_sphere_sst.send(msgs::SstArray {
                                header: header.clone(),
                                sources: sources
                                    .iter()
                                    .map(|track| {
                                        let position = track.direction.to_unit_vec();
                                        msgs::Sst {
                                            id: track.id,
                                            P: track.power,
                                            x: position.x,
                                            y: position.y,
                                            z: position.z,
                                            azimuth: track.direction.azimuth,
                                            elevation: track.direction.elevation,
                                        }
                                    })
                                    .collect(),
                            }),
                            "error sending the unit sphere sst message: {err}"
                        );
                    }
                    #[cfg(feature = "odas-msgs")]
                    if odas_unit_sphere_sst.has_subscribers() {
                        log_error!(
                            odas_unit_sphere_sst.send(msgs::OdasSstArrayStamped {
                                header: header.clone(),
                                sources: sources
                                    .iter()
                                    .map(|track| {
                                        let position = track.direction.to_unit_vec();
                                        msgs::OdasSst {
                                            id: track.id,
                                            activity: track.power,
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
                    #[cfg(feature = "odas-msgs")]
                    let sst_poses_subbed =
                        sst_poses_subbed || odas_unit_sphere_sst_poses.has_subscribers();

                    if sst_poses_subbed {
                        let poses = msgs::PoseArray {
                            header: header.clone(),
                            poses: sources
                                .iter()
                                .map(|track| {
                                    let quaternion = track.direction.to_quaternion().coords;
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
                        #[cfg(feature = "odas-msgs")]
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
                        for (idx, track) in sources.iter().enumerate() {
                            let rotation = track.direction.to_quaternion().coords;
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
                                        x: 1. * track.power / 8000.,
                                        y: 0.1,
                                        z: 0.1,
                                    },
                                    action: msgs::Marker::ADD as i32,
                                    lifetime: rosrust::Duration::from_seconds(1),
                                    ..Default::default()
                                }),
                                "error sending marker {err}"
                            );
                        }
                    }
                    if sss_subbed {
                        let mut channels = Vec::new();
                        let mut mapping = Vec::new();
                        sources.sort_by_key(|t| t.channel.expect("all channels are set"));
                        let length = das.expected_len(&audio);
                        for track in sources {
                            let channel = track.channel.unwrap();
                            assert!(
                                channels.len() <= channel,
                                "channels should be sorted correctly"
                            );
                            // insert empty data for unused channels
                            channels.extend(
                                iter::repeat(vec![0.0; length]).take(channels.len() - channel),
                            );
                            mapping.extend(iter::repeat(-1).take(channels.len() - channel));
                            let data = das.beam_form(track.direction, &audio).collect_vec();
                            assert_eq!(data.len(), length);
                            channels.push(data);
                            mapping.push(track.id);
                        }

                        let audio = Audio::from_channels(audio.sample_rate(), channels);
                        // TODO consider supporting more than one output format.
                        let audio = msgs::AudioData {
                            data: audio.to_interleaved().flat_map(f32::to_le_bytes).collect(),
                        };
                        log_error!(
                            sss_audio_info_topic.send(msgs::AudioInfo {
                                channels: config.channels as u8,
                                sample_rate: config.rate.into(),
                                sample_format: "FLOAT32LE".into(),
                                bitrate: (size_of::<f32>() as u32) * 8 * config.rate as u32,
                                coding_format: "wave".into(),
                            }),
                            "error sending sss audio info message {err}"
                        );
                        log_error!(
                            sss_mapping.send(msgs::SssMapping {
                                header: header.clone(),
                                sources: mapping
                            }),
                            "error sending sss channel mapping message {err}"
                        );
                        #[cfg(feature = "audio_common_msgs-stamped")]
                        log_error!(
                            sss_audio_stamped_topic.send(msgs::AudioDataStamped {
                                header: header.clone(),
                                audio: audio.clone()
                            }),
                            "error sending sss audio message {err}"
                        );
                        log_error!(
                            sss_audio_topic.send(audio),
                            "error sending sss audio message {err}"
                        );
                    }
                }
            }
        }
        Ok(())
    }
}
