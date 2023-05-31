use std::f64::consts::PI;
use std::iter;

use alsa::device_name::{Hint, HintIter};
use alsa::pcm::HwParams;
use alsa::{Direction, PCM};
use nalgebra::vector;
use rosrust::ros_info;
use rosrust_dynamic_reconfigure::{Group, GroupType, Property, Type, Value, Variant};
use ssloc::{Format, MbssConfig, Position};

#[derive(Debug, Clone, PartialEq)]
pub struct Device {
    pub name: String,
    pub description: String,
    pub channels: (u16, u16),
    pub rate: (u16, u16),
    pub formats: Vec<Format>,
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
pub struct Config {
    pub format: Format,
    pub rate: u16,
    pub use_audio_messages: bool,
    pub audio_message_topic: String,
    pub device: Device,
    pub devices: Vec<Device>,
    pub localisation_frame: f64,
    pub channels: u16,
    pub mics: Vec<(Position, bool)>,
    pub max_sources: u16,
    pub mbss: MbssConfig,
    pub mbss_ssl_threashold: f64,
}
impl Config {
    pub fn init() -> rosrust::api::error::Result<Config> {
        let devices: Vec<_> = HintIter::new_str(None, "pcm")
            .map_err(|e| e.to_string())?
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
                        formats: ssloc::Format::supported(&params).collect(),
                    })
                },
            )
            .collect();
        Ok(Config {
            format: devices[0].formats[0],
            rate: devices[0].rate.0,
            use_audio_messages: false,
            audio_message_topic: String::new(),
            device: devices[0].clone(),
            localisation_frame: 1.0,
            channels: devices[0].channels.0,
            devices,
            mics: vec![(vector!(0., 0., 0.), true); 20],
            max_sources: 5,
            mbss: MbssConfig::default(),
            mbss_ssl_threashold: 5000.,
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
        groups.extend((self.channels..self.mics.len() as u16).map(|c| Group {
            name: format!("Mic {c}"),
            state: false,
            id: MIC_GROUP + 1 + c as i32,
            parent: MIC_GROUP,
            type_: GroupType::Hide,
        }));
        groups
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property::new("recording/use_audio_messages", self.use_audio_messages).group(AUDIO_GROUP),
            Property::new("recording/audio_message_topic", &self.audio_message_topic).group(AUDIO_GROUP),
            Property::new_enum("recording/device", &self.device.name, &self.devices).group(AUDIO_GROUP),
            Property::new_range("recording/rate", self.rate, self.device.rate.0, self.device.rate.1)
                .group(AUDIO_GROUP),
            Property::new_enum(
                "recording/format",
                self.format.to_string(),
                self.device.formats.iter().map(ToString::to_string),
            )
            .group(AUDIO_GROUP),
            Property::new_default_range(
                "recording/frame_length",
                self.localisation_frame,
                0.3,
                0.05,
                10.,
            )
            .group(AUDIO_GROUP),
            Property::new_range(
                "recording/channels",
                self.channels,
                self.device.channels.0,
                self.device.channels.1,
            )
            .group(AUDIO_GROUP),
            Property::new_enum("mbss/pooling", "max", ["max", "sum"]).group(MBSS_GROUP),
            Property::new_default_range(
                "mbss/ssl_threashold",
                self.mbss_ssl_threashold,
                5_000.,
                1.,
                10_000.,
            )
            .group(MBSS_GROUP),
            // TODO spectrum_method
            Property::new_default_range("mbss/azimuth/min", self.mbss.azimuth_range.0, -PI, -PI, PI)
                .group(MBSS_GROUP),
            Property::new_default_range("mbss/azimuth/max", self.mbss.azimuth_range.1, PI, -PI, PI)
                .group(MBSS_GROUP),
            Property::new_default_range(
                "mbss/elevation/min",
                self.mbss.elevation_range.0,
                -PI / 2.,
                -PI / 2.,
                PI / 2.,
            )
            .group(MBSS_GROUP),
            Property::new_default_range(
                "mbss/elevation/max",
                self.mbss.elevation_range.1,
                PI / 2.,
                -PI / 2.,
                PI / 2.,
            )
            .group(MBSS_GROUP),
            Property::new_default_range(
                "mbss/grid_res",
                self.mbss.grid_res,
                0.02,
                0.01,
                0.5,
            )
            .group(MBSS_GROUP),
            Property::new_default_range(
                "mbss/alpha_res",
                self.mbss.alpha_res,
                0.02,
                0.01,
                0.5,
            )
            .group(MBSS_GROUP),
            Property::new_default_range(
                "mbss/min_angle",
                self.mbss.min_angle,
                0.1,
                0.01,
                0.5,
            )
            .description("minimal angle between two audio sources")
            .group(MBSS_GROUP),
            Property::new_default_range("mbss/max_sources", self.max_sources, 5, 1, 20)
                .description("maximal number of detected sources")
                .group(MBSS_GROUP),
        ];
        props.extend(
            self.mics
                .iter()
                .enumerate()
                .flat_map(|(idx, (mic, enabled))| {
                    vec![
                        Property::new_range(format_args!("mic/{idx}/x"), mic.x, -2., 2.)
                            .group(MIC_GROUP + 1 + idx as i32),
                        Property::new_range(format_args!("mic/{idx}/y"), mic.y, -2., 2.)
                            .group(MIC_GROUP + 1 + idx as i32),
                        Property::new_range(format_args!("mic/{idx}/z"), mic.z, -2., 2.)
                            .group(MIC_GROUP + 1 + idx as i32),
                        Property::new_default(format_args!("mic/{idx}/enabled"), *enabled, true)
                            .group(MIC_GROUP + 1 + idx as i32),
                    ]
                }),
        );
        props
    }

    fn set(&mut self, name: &str, value: Value) -> rosrust::error::Result<()> {
        ros_info!("Setting: {name}={value}");
        match name {
            "recording/use_audio_messages" => self.use_audio_messages = value.as_bool(name)?,
            "recording/audio_message_topic" => self.audio_message_topic = value.as_string(name)?,
            "recording/device" => {
                let value = value.as_string(name)?;
                self.device = self
                    .devices
                    .iter()
                    .find(|d| d.name == value)
                    .ok_or_else(|| format!("unknown device {value}"))?
                    .clone();
            }
            "recording/rate" => self.rate = value.as_int(name)? as u16,
            "recording/format" => self.format = value.as_string(name)?.parse()?,
            "recording/frame_length" => self.localisation_frame = value.as_float(name)?,
            "recording/channels" => self.channels = value.as_int(name)? as u16,
            mic if mic.starts_with("mic/") => {
                let (idx, coord) = mic
                    .strip_prefix("mic/")
                    .unwrap()
                    .split_once('/')
                    .ok_or_else(|| format!("invalid format for mic coordinate {mic}"))?;
                let idx: usize = idx
                    .parse()
                    .map_err(|e| format!("invalid index for mic coordinate {e}"))?;
                if idx > self.mics.len() {
                    return Err(format!("invalid index for mic coordinate {idx}").into());
                }
                if coord == "enabled" {
                    self.mics[idx].1 = value.as_bool(name)?;
                } else {
                    let value = value.as_float(name)?;
                    match coord {
                        "x" => self.mics[idx].0.x = value,
                        "y" => self.mics[idx].0.y = value,
                        "z" => self.mics[idx].0.z = value,
                        o => return Err(format!("unexpected coordinate: {o}").into()),
                    }
                }
            }
            "mbss/pooling" => self.mbss.pooling = value.as_string(name)?.parse()?,
            "mbss/azimuth/min" => self.mbss.azimuth_range.0 = value.as_float(name)?,
            "mbss/azimuth/max" => self.mbss.azimuth_range.1 = value.as_float(name)?,
            "mbss/elevation/min" => self.mbss.elevation_range.0 = value.as_float(name)?,
            "mbss/elevation/max" => self.mbss.elevation_range.1 = value.as_float(name)?,
            "mbss/grid_res" => self.mbss.grid_res = value.as_float(name)?,
            "mbss/alpha_res" => self.mbss.alpha_res = value.as_float(name)?,
            "mbss/min_angle" => self.mbss.min_angle = value.as_float(name)?,
            "mbss/max_sources" => self.max_sources = value.as_int(name)? as u16,
            "mbss/ssl_threashold" => self.mbss_ssl_threashold = value.as_float(name)?,
            other => return Err(format!("unexpected field: {other}").into()),
        }
        Ok(())
    }
}
