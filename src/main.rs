use std::f64::consts::PI;
use std::time::Duration;
use std::{iter, thread};

use alsa::device_name::{Hint, HintIter};
use alsa::pcm::HwParams;
use alsa::{Direction, PCM};
use lib::{for_format, AudioRecorder, Format, MbssConfig, Position};
use nalgebra::{vector, UnitQuaternion};
use rosrust::{ros_err, ros_info};
use rosrust_dynamic_reconfigure::{Group, GroupType, Property, Type, Value, Variant};

mod msgs {
    pub use rosrust_msg::geometry_msgs::*;
    pub use rosrust_msg::std_msgs::{ColorRGBA, Header};
    pub use rosrust_msg::visualization_msgs::*;
}

#[derive(Debug, Clone, PartialEq)]
struct Device {
    name: String,
    description: String,
    channels: (u16, u16),
    rate: (u16, u16),
    formats: Vec<Format>,
}

impl From<&Device> for Variant {
    fn from(value: &Device) -> Self {
        Variant {
            name: value.description.clone(),
            type_: Type::String,
            value: (&value.name).into(),
            description: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Config {
    format: Format,
    rate: u16,
    device: Device,
    devices: Vec<Device>,
    localisation_frame: f64,
    channels: u16,
    mics: Vec<Position>,
    max_sources: u16,
    mbss: MbssConfig,
}
impl Config {
    fn init() -> anyhow::Result<Config> {
        let devices: Vec<_> = HintIter::new_str(None, "pcm")?
            .chain(iter::once(Hint {
                name: Some("default".into()),
                desc: Some("System Default".into()),
                direction: Some(Direction::Capture),
            }))
            .filter_map(
                |Hint {
                     name,
                     direction,
                     desc,
                 }| {
                    let name = name?;
                    if !matches!(direction, Some(Direction::Capture)) {
                        return None;
                    }
                    let pcm = PCM::new(name.as_str(), Direction::Capture, false).ok()?;
                    let params = HwParams::any(&pcm).ok()?;
                    let channels = params
                        .get_channels()
                        .ok()
                        .map(|c| (c as u16, c as u16))
                        .or_else(|| {
                            Some((
                                params.get_channels_min().ok()? as u16,
                                params.get_channels_max().ok()?.min(20) as u16,
                            ))
                        })?;
                    let rate = params
                        .get_rate()
                        .ok()
                        .map(|c| (c as u16, c as u16))
                        .or_else(|| {
                            Some((
                                params.get_rate_min().ok()? as u16,
                                params.get_rate_max().ok()? as u16,
                            ))
                        })?;
                    Some(Device {
                        name,
                        description: desc.unwrap_or_default(),
                        channels,
                        rate,
                        formats: lib::Format::supported(&params).collect(),
                    })
                },
            )
            .collect();
        Ok(Config {
            format: devices[0].formats[0],
            rate: devices[0].rate.0,
            device: devices[0].clone(),
            channels: devices[0].channels.0,
            devices,
            localisation_frame: 1.0,
            mics: vec![vector!(0., 0., 0.); 20],
            max_sources: 5,
            mbss: MbssConfig::default(),
        })
    }
}

const AUDIO_GROUP: i32 = 100;
const MIC_GROUP: i32 = 200;
const MBSS_GROUP: i32 = 300;

impl rosrust_dynamic_reconfigure::Config for Config {
    fn clean_up(&mut self) {
        if !self.devices.contains(&self.device) {
            self.device = self.devices[0].clone();
        }
        if !self.device.formats.contains(&self.format) {
            self.format = self.device.formats[0];
        }
        self.rate = self.rate.clamp(self.device.rate.0, self.device.rate.1);
        self.channels = self
            .channels
            .clamp(self.device.channels.0, self.device.channels.1);
    }

    fn groups(&self) -> Vec<Group> {
        let mut groups = vec![
            Group {
                name: "default".into(),
                state: false,
                id: 0,
                parent: 0,
                type_: GroupType::Apply,
            },
            Group {
                name: "Audio".into(),
                state: false,
                id: AUDIO_GROUP,
                parent: 0,
                type_: GroupType::Tab,
            },
            Group {
                name: "Microphones".into(),
                state: false,
                id: MIC_GROUP,
                parent: 0,
                type_: GroupType::Tab,
            },
            Group {
                name: "MBSS Settings".into(),
                state: false,
                id: MBSS_GROUP,
                parent: 0,
                type_: GroupType::Tab,
            },
        ];
        groups.extend((0..self.channels).map(|c| Group {
            name: format!("Mic {c}"),
            state: false,
            id: MIC_GROUP + 1 + c as i32,
            parent: MIC_GROUP,
            type_: GroupType::Tab,
        }));
        groups
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property::new_enum("device", &self.device.name, &self.devices).group(AUDIO_GROUP),
            Property::new_range("rate", self.rate, self.device.rate.0, self.device.rate.1)
                .group(AUDIO_GROUP),
            Property::new_enum(
                "format",
                self.format.to_string(),
                self.device.formats.iter().map(ToString::to_string),
            )
            .group(AUDIO_GROUP),
            Property::new_default_range(
                "localisation_frame",
                self.localisation_frame,
                1.0,
                0.1,
                10.,
            )
            .group(AUDIO_GROUP),
            Property::new_range(
                "channels",
                self.channels,
                self.device.channels.0,
                self.device.channels.1,
            )
            .group(AUDIO_GROUP),
            Property::new_enum("pooling", "max", ["max", "sum"]).group(MBSS_GROUP),
            // TODO spectrum_method
            Property::new_default_range("azimuth_min", self.mbss.azimuth_range.0, -PI, -PI, PI)
                .group(MBSS_GROUP),
            Property::new_default_range("azimuth_max", self.mbss.azimuth_range.1, PI, -PI, PI)
                .group(MBSS_GROUP),
            Property::new_default_range(
                "elevation_min",
                self.mbss.elevation_range.0,
                -PI / 2.,
                -PI / 2.,
                PI / 2.,
            )
            .group(MBSS_GROUP),
            Property::new_default_range(
                "elevation_max",
                self.mbss.elevation_range.1,
                PI / 2.,
                -PI / 2.,
                PI / 2.,
            )
            .group(MBSS_GROUP),
            Property::new_default_range(
                "grid_res",
                self.mbss.grid_res,
                1f64.to_radians(),
                0.1f64.to_radians(),
                10f64.to_radians(),
            )
            .group(MBSS_GROUP),
            Property::new_default_range(
                "alpha_res",
                self.mbss.alpha_res,
                1f64.to_radians(),
                0.1f64.to_radians(),
                10f64.to_radians(),
            )
            .group(MBSS_GROUP),
            Property::new_default_range(
                "min_angle",
                self.mbss.min_angle,
                5f64.to_radians(),
                1f64.to_radians(),
                20f64.to_radians(),
            )
            .description("minimal angle between two audio sources")
            .group(MBSS_GROUP),
            Property::new_default_range(
                "max_sources",
                self.max_sources,
                5,
                1,
                20,
            )
            .description("maximal number of detected sources")
            .group(MBSS_GROUP),
        ];
        props.extend(self.mics.iter().enumerate().flat_map(|(idx, mic)| {
            vec![
                Property::new_range(format_args!("mic_{idx}_x"), mic.x, -2., 2.)
                    .group(MIC_GROUP + 1 + idx as i32),
                Property::new_range(format_args!("mic_{idx}_y"), mic.y, -2., 2.)
                    .group(MIC_GROUP + 1 + idx as i32),
                Property::new_range(format_args!("mic_{idx}_z"), mic.z, -2., 2.)
                    .group(MIC_GROUP + 1 + idx as i32),
            ]
        }));
        props
    }

    fn set(&mut self, name: &str, value: Value) -> rosrust::error::Result<()> {
        ros_info!("Setting: {name}={value}");
        match name {
            "format" => self.format = value.as_string("format")?.parse()?,
            "rate" => self.rate = value.as_int(name)? as u16,
            "device" => {
                let value = value.as_string(name)?;
                self.device = self
                    .devices
                    .iter()
                    .find(|d| d.name == value)
                    .ok_or_else(|| format!("unknown device {value}"))?
                    .clone();
            }
            "channels" => self.channels = value.as_int(name)? as u16,
            "localisation_frame" => self.localisation_frame = value.as_float(name)?,
            mic if mic.starts_with("mic_") => {
                let value = value.as_float(name)?;
                let (idx, coord) = mic
                    .strip_prefix("mic_")
                    .unwrap()
                    .split_once('_')
                    .ok_or_else(|| format!("invalid format for mic coordinate {mic}"))?;
                let idx: usize = idx
                    .parse()
                    .map_err(|e| format!("invalid index for mic coordinate {e}"))?;
                if idx > self.mics.len() {
                    return Err(format!("invalid index for mic coordinate {idx}").into());
                }
                // self.mics
                //     .extend((self.mics.len()..=idx).map(|_| Position::default()));
                match coord {
                    "x" => self.mics[idx].x = value,
                    "y" => self.mics[idx].y = value,
                    "z" => self.mics[idx].z = value,
                    o => return Err(format!("unexpected coordinate: {o}").into()),
                }
            }
            "pooling" => self.mbss.pooling = value.as_string(name)?.parse()?,
            "azimuth_min" => self.mbss.azimuth_range.0 = value.as_float(name)?,
            "azimuth_max" => self.mbss.azimuth_range.1 = value.as_float(name)?,
            "elevation_min" => self.mbss.elevation_range.0 = value.as_float(name)?,
            "elevation_max" => self.mbss.elevation_range.1 = value.as_float(name)?,
            "grid_res" => self.mbss.grid_res = value.as_float(name)?,
            "alpha_res" => self.mbss.alpha_res = value.as_float(name)?,
            "min_angle" => self.mbss.min_angle = value.as_float(name)?,
            "max_sources" => self.max_sources = value.as_int(name)? as u16,
            other => return Err(format!("unexpected field: {other}").into()),
        }
        Ok(())
    }
}

fn main() {
    env_logger::init();

    // Initialize node
    rosrust::init("ssloc");

    // Create publisher

    // let log_names = rosrust::param("~log_names").unwrap().get().unwrap_or(false);

    let mut config_server =
        rosrust_dynamic_reconfigure::Server::<Config>::new(Config::init().unwrap()).unwrap();

    let updating_config = config_server.get_config_updating();

    thread::spawn(move || {
        let markers = rosrust::publish("~markers", 20).unwrap();

        let mut config = updating_config.copy();
        let mbss_config = MbssConfig::default();

        'recorder: while rosrust::is_ok() {
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

                'mbss: while rosrust::is_ok() {
                    let mbss = config.mbss.create(config.mics[..config.channels as usize].to_owned());
                    while rosrust::is_ok() {
                        let max_sources = {
                            let update = updating_config.read();
                            if update.channels != config.channels
                                || update.device != config.device
                                || update.rate != config.rate
                                || update.format != config.format
                                || update.localisation_frame != config.localisation_frame
                            {
                                config = update.clone();
                                continue 'recorder;
                            }
                            if update.mics != config.mics || update.mbss != config.mbss {
                                config = update.clone();
                                continue 'mbss;
                            }
                            update.max_sources.into()
                        };
                        let sources = mbss.locate_spec(
                            match &recorder.record() {
                                Ok(audio) => audio,
                                Err(err) => {
                                    ros_err!("error recording audio {err}");
                                    continue 'recorder;
                                }
                            },
                            max_sources,
                        );
                        for (idx, (az, el, strength)) in sources.into_iter().enumerate() {
                            ros_info!("{strength}");
                            let rotation = UnitQuaternion::from_euler_angles(0., -el, az).coords;
                            if let Err(e) = markers.send(msgs::Marker {
                                header: msgs::Header {
                                    stamp: rosrust::now(),
                                    frame_id: "ssloc".to_string(),
                                    ..Default::default()
                                },
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
                                     x: (strength / 6000.).clamp(0.2, 2.),
                                    // x: 1.,
                                    y: 0.2,
                                    z: 0.2,
                                },
                                action: msgs::Marker::ADD as i32,
                                lifetime: rosrust::Duration::from_seconds(1),
                                ..Default::default()
                            }) {
                                ros_err!("error sending marker {e}");
                            };
                        }
                    }
                }
            })
        }
    });

    // Create object that maintains 10Hz between sleep requests
    let rate = rosrust::rate(10.0);

    // Breaks when a shutdown signal is sent
    while rosrust::is_ok() {
        // // Create string message
        // let msg = rosrust_msg::std_msgs::String {
        //     data: format!("hello world from rosrust {}", count),
        // };

        // Log event
        // rosrust::ros_info!("Publishing: {:?}", msg);

        // Send string message to topic via publisher

        // if log_names {
        //     rosrust::ros_info!("Subscriber names: {:?}",
        // chatter_pub.subscriber_names()); }

        // Sleep to maintain 10Hz rate
        rate.sleep();
    }
}
