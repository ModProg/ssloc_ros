#![allow(clippy::all, unused)]
pub mod ssloc_ros_msgs {
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct SslArray {
        pub header: super::std_msgs::Header,
        pub sources: Vec<super::ssloc_ros_msgs::Ssl>,
    }
    impl SslArray {}

    impl std::convert::From<SslArray> for rosrust::MsgValue {
        fn from(src: SslArray) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<SslArray> for rosrust::MsgMessage {
        fn from(src: SslArray) -> Self {
            let mut output = Self::new();
            output.insert("header".into(), src.header.into());
            output.insert("sources".into(), src.sources.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for SslArray {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for SslArray {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                header: src.remove("header").ok_or(())?.try_into()?,
                sources: src.remove("sources").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for SslArray {
        fn eq(&self, other: &Self) -> bool {
            true && self.header == other.header && self.sources == other.sources
        }
    }
    impl std::fmt::Debug for SslArray {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(SslArray))
                .field(stringify!(header), &self.header)
                .field(stringify!(sources), &self.sources)
                .finish()
        }
    }
    impl Default for SslArray {
        fn default() -> Self {
            Self {
                header: Default::default(),
                sources: Default::default(),
            }
        }
    }
    impl rosrust::Message for SslArray {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "# A list of SSLs\nHeader header\nssloc_ros_msgs/Ssl[] sources\n\n================================================================================\nMSG: std_msgs/Header\n# Standard metadata for higher-level stamped data types.\n# This is generally used to communicate timestamped data \n# in a particular coordinate frame.\n# \n# sequence ID: consecutively increasing ID \nuint32 seq\n#Two-integer timestamp that is expressed as:\n# * stamp.sec: seconds (stamp_secs) since epoch (in Python the variable is called 'secs')\n# * stamp.nsec: nanoseconds since stamp_secs (in Python the variable is called 'nsecs')\n# time-handling sugar is provided by the client library\ntime stamp\n#Frame this data is associated with\nstring frame_id\n\n================================================================================\nMSG: ssloc_ros_msgs/Ssl\n# A single SSL on the unit sphere, x, y, z describe the same point as `azimuth` and `elevation`\n# P is the cross correlation at that DoA\nfloat64 x\nfloat64 y\nfloat64 z\nfloat64 azimuth\nfloat64 elevation\nfloat64 P\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "ca09479cd35f89fa65817ba7ef2797ed".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "ssloc_ros_msgs/SslArray".into()
        }

        fn set_header(
            &mut self,
            clock: &::std::sync::Arc<dyn rosrust::Clock>,
            seq: &::std::sync::Arc<::std::sync::atomic::AtomicUsize>,
        ) {
            if self.header.seq == 0 {
                self.header.seq = seq.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst) as u32;
            }
            if self.header.stamp.nanos() == 0 {
                self.header.stamp = clock.now();
            }
        }
    }
    impl rosrust::rosmsg::RosMsg for SslArray {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.header.encode(w.by_ref())?;
            rosrust::rosmsg::encode_variable_slice(&self.sources, w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                header: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                sources: rosrust::rosmsg::decode_variable_vec(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct Sst {
        pub id: i64,
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub azimuth: f64,
        pub elevation: f64,
        pub P: f64,
    }
    impl Sst {}

    impl std::convert::From<Sst> for rosrust::MsgValue {
        fn from(src: Sst) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<Sst> for rosrust::MsgMessage {
        fn from(src: Sst) -> Self {
            let mut output = Self::new();
            output.insert("id".into(), src.id.into());
            output.insert("x".into(), src.x.into());
            output.insert("y".into(), src.y.into());
            output.insert("z".into(), src.z.into());
            output.insert("azimuth".into(), src.azimuth.into());
            output.insert("elevation".into(), src.elevation.into());
            output.insert("P".into(), src.P.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for Sst {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for Sst {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                id: src.remove("id").ok_or(())?.try_into()?,
                x: src.remove("x").ok_or(())?.try_into()?,
                y: src.remove("y").ok_or(())?.try_into()?,
                z: src.remove("z").ok_or(())?.try_into()?,
                azimuth: src.remove("azimuth").ok_or(())?.try_into()?,
                elevation: src.remove("elevation").ok_or(())?.try_into()?,
                P: src.remove("P").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for Sst {
        fn eq(&self, other: &Self) -> bool {
            true && self.id == other.id
                && self.x == other.x
                && self.y == other.y
                && self.z == other.z
                && self.azimuth == other.azimuth
                && self.elevation == other.elevation
                && self.P == other.P
        }
    }
    impl std::fmt::Debug for Sst {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(Sst))
                .field(stringify!(id), &self.id)
                .field(stringify!(x), &self.x)
                .field(stringify!(y), &self.y)
                .field(stringify!(z), &self.z)
                .field(stringify!(azimuth), &self.azimuth)
                .field(stringify!(elevation), &self.elevation)
                .field(stringify!(P), &self.P)
                .finish()
        }
    }
    impl Default for Sst {
        fn default() -> Self {
            Self {
                id: Default::default(),
                x: Default::default(),
                y: Default::default(),
                z: Default::default(),
                azimuth: Default::default(),
                elevation: Default::default(),
                P: Default::default(),
            }
        }
    }
    impl rosrust::Message for Sst {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "# A single SST on the unit sphere, x, y, z describe the same point as `azimuth` and \
             `elevation`\n# id is a unique id for this track, persistent over multiple frames\n# P \
             is the cross correlation at that DoA\nint64 id\nfloat64 x\nfloat64 y\nfloat64 \
             z\nfloat64 azimuth\nfloat64 elevation\nfloat64 P\n"
                .into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "10e2154b185d67d9990be400740bab32".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "ssloc_ros_msgs/Sst".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for Sst {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.id.encode(w.by_ref())?;
            self.x.encode(w.by_ref())?;
            self.y.encode(w.by_ref())?;
            self.z.encode(w.by_ref())?;
            self.azimuth.encode(w.by_ref())?;
            self.elevation.encode(w.by_ref())?;
            self.P.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                id: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                x: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                y: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                z: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                azimuth: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                elevation: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                P: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct SssMapping {
        pub header: super::std_msgs::Header,
        pub sources: Vec<i64>,
    }
    impl SssMapping {}

    impl std::convert::From<SssMapping> for rosrust::MsgValue {
        fn from(src: SssMapping) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<SssMapping> for rosrust::MsgMessage {
        fn from(src: SssMapping) -> Self {
            let mut output = Self::new();
            output.insert("header".into(), src.header.into());
            output.insert("sources".into(), src.sources.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for SssMapping {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for SssMapping {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                header: src.remove("header").ok_or(())?.try_into()?,
                sources: src.remove("sources").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for SssMapping {
        fn eq(&self, other: &Self) -> bool {
            true && self.header == other.header && self.sources == other.sources
        }
    }
    impl std::fmt::Debug for SssMapping {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(SssMapping))
                .field(stringify!(header), &self.header)
                .field(stringify!(sources), &self.sources)
                .finish()
        }
    }
    impl Default for SssMapping {
        fn default() -> Self {
            Self {
                header: Default::default(),
                sources: Default::default(),
            }
        }
    }
    impl rosrust::Message for SssMapping {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "# Mapps each channel in the SSS audio messages `idx` to the corresponding SST id `sources[idx]`\nHeader header\nint64[] sources\n\n================================================================================\nMSG: std_msgs/Header\n# Standard metadata for higher-level stamped data types.\n# This is generally used to communicate timestamped data \n# in a particular coordinate frame.\n# \n# sequence ID: consecutively increasing ID \nuint32 seq\n#Two-integer timestamp that is expressed as:\n# * stamp.sec: seconds (stamp_secs) since epoch (in Python the variable is called 'secs')\n# * stamp.nsec: nanoseconds since stamp_secs (in Python the variable is called 'nsecs')\n# time-handling sugar is provided by the client library\ntime stamp\n#Frame this data is associated with\nstring frame_id\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "7001f5ee86e256ffc77c697015d3bd70".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "ssloc_ros_msgs/SssMapping".into()
        }

        fn set_header(
            &mut self,
            clock: &::std::sync::Arc<dyn rosrust::Clock>,
            seq: &::std::sync::Arc<::std::sync::atomic::AtomicUsize>,
        ) {
            if self.header.seq == 0 {
                self.header.seq = seq.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst) as u32;
            }
            if self.header.stamp.nanos() == 0 {
                self.header.stamp = clock.now();
            }
        }
    }
    impl rosrust::rosmsg::RosMsg for SssMapping {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.header.encode(w.by_ref())?;
            rosrust::rosmsg::encode_variable_primitive_slice(&self.sources, w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                header: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                sources: rosrust::rosmsg::decode_variable_primitive_vec(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct Ssl {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub azimuth: f64,
        pub elevation: f64,
        pub P: f64,
    }
    impl Ssl {}

    impl std::convert::From<Ssl> for rosrust::MsgValue {
        fn from(src: Ssl) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<Ssl> for rosrust::MsgMessage {
        fn from(src: Ssl) -> Self {
            let mut output = Self::new();
            output.insert("x".into(), src.x.into());
            output.insert("y".into(), src.y.into());
            output.insert("z".into(), src.z.into());
            output.insert("azimuth".into(), src.azimuth.into());
            output.insert("elevation".into(), src.elevation.into());
            output.insert("P".into(), src.P.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for Ssl {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for Ssl {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                x: src.remove("x").ok_or(())?.try_into()?,
                y: src.remove("y").ok_or(())?.try_into()?,
                z: src.remove("z").ok_or(())?.try_into()?,
                azimuth: src.remove("azimuth").ok_or(())?.try_into()?,
                elevation: src.remove("elevation").ok_or(())?.try_into()?,
                P: src.remove("P").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for Ssl {
        fn eq(&self, other: &Self) -> bool {
            true && self.x == other.x
                && self.y == other.y
                && self.z == other.z
                && self.azimuth == other.azimuth
                && self.elevation == other.elevation
                && self.P == other.P
        }
    }
    impl std::fmt::Debug for Ssl {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(Ssl))
                .field(stringify!(x), &self.x)
                .field(stringify!(y), &self.y)
                .field(stringify!(z), &self.z)
                .field(stringify!(azimuth), &self.azimuth)
                .field(stringify!(elevation), &self.elevation)
                .field(stringify!(P), &self.P)
                .finish()
        }
    }
    impl Default for Ssl {
        fn default() -> Self {
            Self {
                x: Default::default(),
                y: Default::default(),
                z: Default::default(),
                azimuth: Default::default(),
                elevation: Default::default(),
                P: Default::default(),
            }
        }
    }
    impl rosrust::Message for Ssl {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "# A single SSL on the unit sphere, x, y, z describe the same point as `azimuth` and \
             `elevation`\n# P is the cross correlation at that DoA\nfloat64 x\nfloat64 y\nfloat64 \
             z\nfloat64 azimuth\nfloat64 elevation\nfloat64 P\n"
                .into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "f2382ebd7ac831e964a76978a42e95e2".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "ssloc_ros_msgs/Ssl".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for Ssl {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.x.encode(w.by_ref())?;
            self.y.encode(w.by_ref())?;
            self.z.encode(w.by_ref())?;
            self.azimuth.encode(w.by_ref())?;
            self.elevation.encode(w.by_ref())?;
            self.P.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                x: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                y: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                z: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                azimuth: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                elevation: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                P: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct SstArray {
        pub header: super::std_msgs::Header,
        pub sources: Vec<super::ssloc_ros_msgs::Sst>,
    }
    impl SstArray {}

    impl std::convert::From<SstArray> for rosrust::MsgValue {
        fn from(src: SstArray) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<SstArray> for rosrust::MsgMessage {
        fn from(src: SstArray) -> Self {
            let mut output = Self::new();
            output.insert("header".into(), src.header.into());
            output.insert("sources".into(), src.sources.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for SstArray {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for SstArray {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                header: src.remove("header").ok_or(())?.try_into()?,
                sources: src.remove("sources").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for SstArray {
        fn eq(&self, other: &Self) -> bool {
            true && self.header == other.header && self.sources == other.sources
        }
    }
    impl std::fmt::Debug for SstArray {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(SstArray))
                .field(stringify!(header), &self.header)
                .field(stringify!(sources), &self.sources)
                .finish()
        }
    }
    impl Default for SstArray {
        fn default() -> Self {
            Self {
                header: Default::default(),
                sources: Default::default(),
            }
        }
    }
    impl rosrust::Message for SstArray {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "Header header\nssloc_ros_msgs/Sst[] sources\n\n================================================================================\nMSG: std_msgs/Header\n# Standard metadata for higher-level stamped data types.\n# This is generally used to communicate timestamped data \n# in a particular coordinate frame.\n# \n# sequence ID: consecutively increasing ID \nuint32 seq\n#Two-integer timestamp that is expressed as:\n# * stamp.sec: seconds (stamp_secs) since epoch (in Python the variable is called 'secs')\n# * stamp.nsec: nanoseconds since stamp_secs (in Python the variable is called 'nsecs')\n# time-handling sugar is provided by the client library\ntime stamp\n#Frame this data is associated with\nstring frame_id\n\n================================================================================\nMSG: ssloc_ros_msgs/Sst\n# A single SST on the unit sphere, x, y, z describe the same point as `azimuth` and `elevation`\n# id is a unique id for this track, persistent over multiple frames\n# P is the cross correlation at that DoA\nint64 id\nfloat64 x\nfloat64 y\nfloat64 z\nfloat64 azimuth\nfloat64 elevation\nfloat64 P\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "728f02d21a3e1573f476880390d6fe82".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "ssloc_ros_msgs/SstArray".into()
        }

        fn set_header(
            &mut self,
            clock: &::std::sync::Arc<dyn rosrust::Clock>,
            seq: &::std::sync::Arc<::std::sync::atomic::AtomicUsize>,
        ) {
            if self.header.seq == 0 {
                self.header.seq = seq.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst) as u32;
            }
            if self.header.stamp.nanos() == 0 {
                self.header.stamp = clock.now();
            }
        }
    }
    impl rosrust::rosmsg::RosMsg for SstArray {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.header.encode(w.by_ref())?;
            rosrust::rosmsg::encode_variable_slice(&self.sources, w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                header: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                sources: rosrust::rosmsg::decode_variable_vec(r.by_ref())?,
            })
        }
    }
}
pub mod geometry_msgs {
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }
    impl Point {}

    impl std::convert::From<Point> for rosrust::MsgValue {
        fn from(src: Point) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<Point> for rosrust::MsgMessage {
        fn from(src: Point) -> Self {
            let mut output = Self::new();
            output.insert("x".into(), src.x.into());
            output.insert("y".into(), src.y.into());
            output.insert("z".into(), src.z.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for Point {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for Point {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                x: src.remove("x").ok_or(())?.try_into()?,
                y: src.remove("y").ok_or(())?.try_into()?,
                z: src.remove("z").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for Point {
        fn eq(&self, other: &Self) -> bool {
            true && self.x == other.x && self.y == other.y && self.z == other.z
        }
    }
    impl std::fmt::Debug for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(Point))
                .field(stringify!(x), &self.x)
                .field(stringify!(y), &self.y)
                .field(stringify!(z), &self.z)
                .finish()
        }
    }
    impl Default for Point {
        fn default() -> Self {
            Self {
                x: Default::default(),
                y: Default::default(),
                z: Default::default(),
            }
        }
    }
    impl rosrust::Message for Point {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "# This contains the position of a point in free space\nfloat64 x\nfloat64 y\nfloat64 \
             z\n"
            .into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "4a842b65f413084dc2b10fb484ea7f17".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "geometry_msgs/Point".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for Point {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.x.encode(w.by_ref())?;
            self.y.encode(w.by_ref())?;
            self.z.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                x: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                y: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                z: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct PoseArray {
        pub header: super::std_msgs::Header,
        pub poses: Vec<Pose>,
    }
    impl PoseArray {}

    impl std::convert::From<PoseArray> for rosrust::MsgValue {
        fn from(src: PoseArray) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<PoseArray> for rosrust::MsgMessage {
        fn from(src: PoseArray) -> Self {
            let mut output = Self::new();
            output.insert("header".into(), src.header.into());
            output.insert("poses".into(), src.poses.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for PoseArray {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for PoseArray {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                header: src.remove("header").ok_or(())?.try_into()?,
                poses: src.remove("poses").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for PoseArray {
        fn eq(&self, other: &Self) -> bool {
            true && self.header == other.header && self.poses == other.poses
        }
    }
    impl std::fmt::Debug for PoseArray {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(PoseArray))
                .field(stringify!(header), &self.header)
                .field(stringify!(poses), &self.poses)
                .finish()
        }
    }
    impl Default for PoseArray {
        fn default() -> Self {
            Self {
                header: Default::default(),
                poses: Default::default(),
            }
        }
    }
    impl rosrust::Message for PoseArray {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "# An array of poses with a header for global reference.\n\nHeader header\n\nPose[] poses\n\n================================================================================\nMSG: std_msgs/Header\n# Standard metadata for higher-level stamped data types.\n# This is generally used to communicate timestamped data \n# in a particular coordinate frame.\n# \n# sequence ID: consecutively increasing ID \nuint32 seq\n#Two-integer timestamp that is expressed as:\n# * stamp.sec: seconds (stamp_secs) since epoch (in Python the variable is called 'secs')\n# * stamp.nsec: nanoseconds since stamp_secs (in Python the variable is called 'nsecs')\n# time-handling sugar is provided by the client library\ntime stamp\n#Frame this data is associated with\nstring frame_id\n\n================================================================================\nMSG: geometry_msgs/Pose\n# A representation of pose in free space, composed of position and orientation. \nPoint position\nQuaternion orientation\n\n================================================================================\nMSG: geometry_msgs/Point\n# This contains the position of a point in free space\nfloat64 x\nfloat64 y\nfloat64 z\n\n================================================================================\nMSG: geometry_msgs/Quaternion\n# This represents an orientation in free space in quaternion form.\n\nfloat64 x\nfloat64 y\nfloat64 z\nfloat64 w\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "916c28c5764443f268b296bb671b9d97".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "geometry_msgs/PoseArray".into()
        }

        fn set_header(
            &mut self,
            clock: &::std::sync::Arc<dyn rosrust::Clock>,
            seq: &::std::sync::Arc<::std::sync::atomic::AtomicUsize>,
        ) {
            if self.header.seq == 0 {
                self.header.seq = seq.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst) as u32;
            }
            if self.header.stamp.nanos() == 0 {
                self.header.stamp = clock.now();
            }
        }
    }
    impl rosrust::rosmsg::RosMsg for PoseArray {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.header.encode(w.by_ref())?;
            rosrust::rosmsg::encode_variable_slice(&self.poses, w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                header: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                poses: rosrust::rosmsg::decode_variable_vec(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct Pose {
        pub position: Point,
        pub orientation: Quaternion,
    }
    impl Pose {}

    impl std::convert::From<Pose> for rosrust::MsgValue {
        fn from(src: Pose) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<Pose> for rosrust::MsgMessage {
        fn from(src: Pose) -> Self {
            let mut output = Self::new();
            output.insert("position".into(), src.position.into());
            output.insert("orientation".into(), src.orientation.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for Pose {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for Pose {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                position: src.remove("position").ok_or(())?.try_into()?,
                orientation: src.remove("orientation").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for Pose {
        fn eq(&self, other: &Self) -> bool {
            true && self.position == other.position && self.orientation == other.orientation
        }
    }
    impl std::fmt::Debug for Pose {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(Pose))
                .field(stringify!(position), &self.position)
                .field(stringify!(orientation), &self.orientation)
                .finish()
        }
    }
    impl Default for Pose {
        fn default() -> Self {
            Self {
                position: Default::default(),
                orientation: Default::default(),
            }
        }
    }
    impl rosrust::Message for Pose {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "# A representation of pose in free space, composed of position and orientation. \
             \nPoint position\nQuaternion orientation\n\\
             n================================================================================\\
             nMSG: geometry_msgs/Point\n# This contains the position of a point in free \
             space\nfloat64 x\nfloat64 y\nfloat64 \
             z\n\n================================================================================\\
             \
             nMSG: geometry_msgs/Quaternion\n# This represents an orientation in free space in \
             quaternion form.\n\nfloat64 x\nfloat64 y\nfloat64 z\nfloat64 w\n"
                .into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "e45d45a5a1ce597b249e23fb30fc871f".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "geometry_msgs/Pose".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for Pose {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.position.encode(w.by_ref())?;
            self.orientation.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                position: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                orientation: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct Quaternion {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub w: f64,
    }
    impl Quaternion {}

    impl std::convert::From<Quaternion> for rosrust::MsgValue {
        fn from(src: Quaternion) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<Quaternion> for rosrust::MsgMessage {
        fn from(src: Quaternion) -> Self {
            let mut output = Self::new();
            output.insert("x".into(), src.x.into());
            output.insert("y".into(), src.y.into());
            output.insert("z".into(), src.z.into());
            output.insert("w".into(), src.w.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for Quaternion {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for Quaternion {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                x: src.remove("x").ok_or(())?.try_into()?,
                y: src.remove("y").ok_or(())?.try_into()?,
                z: src.remove("z").ok_or(())?.try_into()?,
                w: src.remove("w").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for Quaternion {
        fn eq(&self, other: &Self) -> bool {
            true && self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
        }
    }
    impl std::fmt::Debug for Quaternion {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(Quaternion))
                .field(stringify!(x), &self.x)
                .field(stringify!(y), &self.y)
                .field(stringify!(z), &self.z)
                .field(stringify!(w), &self.w)
                .finish()
        }
    }
    impl Default for Quaternion {
        fn default() -> Self {
            Self {
                x: Default::default(),
                y: Default::default(),
                z: Default::default(),
                w: Default::default(),
            }
        }
    }
    impl rosrust::Message for Quaternion {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "# This represents an orientation in free space in quaternion form.\n\nfloat64 \
             x\nfloat64 y\nfloat64 z\nfloat64 w\n"
                .into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "a779879fadf0160734f906b8c19c7004".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "geometry_msgs/Quaternion".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for Quaternion {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.x.encode(w.by_ref())?;
            self.y.encode(w.by_ref())?;
            self.z.encode(w.by_ref())?;
            self.w.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                x: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                y: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                z: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                w: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct Vector3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }
    impl Vector3 {}

    impl std::convert::From<Vector3> for rosrust::MsgValue {
        fn from(src: Vector3) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<Vector3> for rosrust::MsgMessage {
        fn from(src: Vector3) -> Self {
            let mut output = Self::new();
            output.insert("x".into(), src.x.into());
            output.insert("y".into(), src.y.into());
            output.insert("z".into(), src.z.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for Vector3 {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for Vector3 {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                x: src.remove("x").ok_or(())?.try_into()?,
                y: src.remove("y").ok_or(())?.try_into()?,
                z: src.remove("z").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for Vector3 {
        fn eq(&self, other: &Self) -> bool {
            true && self.x == other.x && self.y == other.y && self.z == other.z
        }
    }
    impl std::fmt::Debug for Vector3 {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(Vector3))
                .field(stringify!(x), &self.x)
                .field(stringify!(y), &self.y)
                .field(stringify!(z), &self.z)
                .finish()
        }
    }
    impl Default for Vector3 {
        fn default() -> Self {
            Self {
                x: Default::default(),
                y: Default::default(),
                z: Default::default(),
            }
        }
    }
    impl rosrust::Message for Vector3 {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "# This represents a vector in free space. \n# It is only meant to represent a \
             direction. Therefore, it does not\n# make sense to apply a translation to it (e.g., \
             when applying a \n# generic rigid transformation to a Vector3, tf2 will only apply \
             the\n# rotation). If you want your data to be translatable too, use the\n# \
             geometry_msgs/Point message instead.\n\nfloat64 x\nfloat64 y\nfloat64 z\n"
                .into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "4a842b65f413084dc2b10fb484ea7f17".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "geometry_msgs/Vector3".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for Vector3 {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.x.encode(w.by_ref())?;
            self.y.encode(w.by_ref())?;
            self.z.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                x: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                y: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                z: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
}
pub mod odas_ros {
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct OdasSst {
        pub id: i64,
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub activity: f64,
    }
    impl OdasSst {}

    impl std::convert::From<OdasSst> for rosrust::MsgValue {
        fn from(src: OdasSst) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<OdasSst> for rosrust::MsgMessage {
        fn from(src: OdasSst) -> Self {
            let mut output = Self::new();
            output.insert("id".into(), src.id.into());
            output.insert("x".into(), src.x.into());
            output.insert("y".into(), src.y.into());
            output.insert("z".into(), src.z.into());
            output.insert("activity".into(), src.activity.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for OdasSst {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for OdasSst {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                id: src.remove("id").ok_or(())?.try_into()?,
                x: src.remove("x").ok_or(())?.try_into()?,
                y: src.remove("y").ok_or(())?.try_into()?,
                z: src.remove("z").ok_or(())?.try_into()?,
                activity: src.remove("activity").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for OdasSst {
        fn eq(&self, other: &Self) -> bool {
            true && self.id == other.id
                && self.x == other.x
                && self.y == other.y
                && self.z == other.z
                && self.activity == other.activity
        }
    }
    impl std::fmt::Debug for OdasSst {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(OdasSst))
                .field(stringify!(id), &self.id)
                .field(stringify!(x), &self.x)
                .field(stringify!(y), &self.y)
                .field(stringify!(z), &self.z)
                .field(stringify!(activity), &self.activity)
                .finish()
        }
    }
    impl Default for OdasSst {
        fn default() -> Self {
            Self {
                id: Default::default(),
                x: Default::default(),
                y: Default::default(),
                z: Default::default(),
                activity: Default::default(),
            }
        }
    }
    impl rosrust::Message for OdasSst {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "int64 id\nfloat64 x\nfloat64 y\nfloat64 z\nfloat64 activity\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "eb4862f514db8f5e751748179ffd36f6".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "odas_ros/OdasSst".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for OdasSst {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.id.encode(w.by_ref())?;
            self.x.encode(w.by_ref())?;
            self.y.encode(w.by_ref())?;
            self.z.encode(w.by_ref())?;
            self.activity.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                id: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                x: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                y: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                z: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                activity: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct OdasSsl {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub E: f64,
    }
    impl OdasSsl {}

    impl std::convert::From<OdasSsl> for rosrust::MsgValue {
        fn from(src: OdasSsl) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<OdasSsl> for rosrust::MsgMessage {
        fn from(src: OdasSsl) -> Self {
            let mut output = Self::new();
            output.insert("x".into(), src.x.into());
            output.insert("y".into(), src.y.into());
            output.insert("z".into(), src.z.into());
            output.insert("E".into(), src.E.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for OdasSsl {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for OdasSsl {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                x: src.remove("x").ok_or(())?.try_into()?,
                y: src.remove("y").ok_or(())?.try_into()?,
                z: src.remove("z").ok_or(())?.try_into()?,
                E: src.remove("E").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for OdasSsl {
        fn eq(&self, other: &Self) -> bool {
            true && self.x == other.x && self.y == other.y && self.z == other.z && self.E == other.E
        }
    }
    impl std::fmt::Debug for OdasSsl {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(OdasSsl))
                .field(stringify!(x), &self.x)
                .field(stringify!(y), &self.y)
                .field(stringify!(z), &self.z)
                .field(stringify!(E), &self.E)
                .finish()
        }
    }
    impl Default for OdasSsl {
        fn default() -> Self {
            Self {
                x: Default::default(),
                y: Default::default(),
                z: Default::default(),
                E: Default::default(),
            }
        }
    }
    impl rosrust::Message for OdasSsl {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "float64 x\nfloat64 y\nfloat64 z\nfloat64 E\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "da04e2eb0c067758372e166e0d3d06bb".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "odas_ros/OdasSsl".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for OdasSsl {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.x.encode(w.by_ref())?;
            self.y.encode(w.by_ref())?;
            self.z.encode(w.by_ref())?;
            self.E.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                x: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                y: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                z: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                E: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct OdasSslArrayStamped {
        pub header: super::std_msgs::Header,
        pub sources: Vec<super::odas_ros::OdasSsl>,
    }
    impl OdasSslArrayStamped {}

    impl std::convert::From<OdasSslArrayStamped> for rosrust::MsgValue {
        fn from(src: OdasSslArrayStamped) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<OdasSslArrayStamped> for rosrust::MsgMessage {
        fn from(src: OdasSslArrayStamped) -> Self {
            let mut output = Self::new();
            output.insert("header".into(), src.header.into());
            output.insert("sources".into(), src.sources.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for OdasSslArrayStamped {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for OdasSslArrayStamped {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                header: src.remove("header").ok_or(())?.try_into()?,
                sources: src.remove("sources").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for OdasSslArrayStamped {
        fn eq(&self, other: &Self) -> bool {
            true && self.header == other.header && self.sources == other.sources
        }
    }
    impl std::fmt::Debug for OdasSslArrayStamped {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(OdasSslArrayStamped))
                .field(stringify!(header), &self.header)
                .field(stringify!(sources), &self.sources)
                .finish()
        }
    }
    impl Default for OdasSslArrayStamped {
        fn default() -> Self {
            Self {
                header: Default::default(),
                sources: Default::default(),
            }
        }
    }
    impl rosrust::Message for OdasSslArrayStamped {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "Header header\nodas_ros/OdasSsl[] sources\n\n================================================================================\nMSG: std_msgs/Header\n# Standard metadata for higher-level stamped data types.\n# This is generally used to communicate timestamped data \n# in a particular coordinate frame.\n# \n# sequence ID: consecutively increasing ID \nuint32 seq\n#Two-integer timestamp that is expressed as:\n# * stamp.sec: seconds (stamp_secs) since epoch (in Python the variable is called 'secs')\n# * stamp.nsec: nanoseconds since stamp_secs (in Python the variable is called 'nsecs')\n# time-handling sugar is provided by the client library\ntime stamp\n#Frame this data is associated with\nstring frame_id\n\n================================================================================\nMSG: odas_ros/OdasSsl\nfloat64 x\nfloat64 y\nfloat64 z\nfloat64 E\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "356f8a800528bac6e1bfe4fc7310b94b".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "odas_ros/OdasSslArrayStamped".into()
        }

        fn set_header(
            &mut self,
            clock: &::std::sync::Arc<dyn rosrust::Clock>,
            seq: &::std::sync::Arc<::std::sync::atomic::AtomicUsize>,
        ) {
            if self.header.seq == 0 {
                self.header.seq = seq.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst) as u32;
            }
            if self.header.stamp.nanos() == 0 {
                self.header.stamp = clock.now();
            }
        }
    }
    impl rosrust::rosmsg::RosMsg for OdasSslArrayStamped {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.header.encode(w.by_ref())?;
            rosrust::rosmsg::encode_variable_slice(&self.sources, w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                header: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                sources: rosrust::rosmsg::decode_variable_vec(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct OdasSstArrayStamped {
        pub header: super::std_msgs::Header,
        pub sources: Vec<super::odas_ros::OdasSst>,
    }
    impl OdasSstArrayStamped {}

    impl std::convert::From<OdasSstArrayStamped> for rosrust::MsgValue {
        fn from(src: OdasSstArrayStamped) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<OdasSstArrayStamped> for rosrust::MsgMessage {
        fn from(src: OdasSstArrayStamped) -> Self {
            let mut output = Self::new();
            output.insert("header".into(), src.header.into());
            output.insert("sources".into(), src.sources.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for OdasSstArrayStamped {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for OdasSstArrayStamped {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                header: src.remove("header").ok_or(())?.try_into()?,
                sources: src.remove("sources").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for OdasSstArrayStamped {
        fn eq(&self, other: &Self) -> bool {
            true && self.header == other.header && self.sources == other.sources
        }
    }
    impl std::fmt::Debug for OdasSstArrayStamped {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(OdasSstArrayStamped))
                .field(stringify!(header), &self.header)
                .field(stringify!(sources), &self.sources)
                .finish()
        }
    }
    impl Default for OdasSstArrayStamped {
        fn default() -> Self {
            Self {
                header: Default::default(),
                sources: Default::default(),
            }
        }
    }
    impl rosrust::Message for OdasSstArrayStamped {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "Header header\nodas_ros/OdasSst[] sources\n\n================================================================================\nMSG: std_msgs/Header\n# Standard metadata for higher-level stamped data types.\n# This is generally used to communicate timestamped data \n# in a particular coordinate frame.\n# \n# sequence ID: consecutively increasing ID \nuint32 seq\n#Two-integer timestamp that is expressed as:\n# * stamp.sec: seconds (stamp_secs) since epoch (in Python the variable is called 'secs')\n# * stamp.nsec: nanoseconds since stamp_secs (in Python the variable is called 'nsecs')\n# time-handling sugar is provided by the client library\ntime stamp\n#Frame this data is associated with\nstring frame_id\n\n================================================================================\nMSG: odas_ros/OdasSst\nint64 id\nfloat64 x\nfloat64 y\nfloat64 z\nfloat64 activity\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "9d2c6de6487e9fc578a7b9e452e8489d".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "odas_ros/OdasSstArrayStamped".into()
        }

        fn set_header(
            &mut self,
            clock: &::std::sync::Arc<dyn rosrust::Clock>,
            seq: &::std::sync::Arc<::std::sync::atomic::AtomicUsize>,
        ) {
            if self.header.seq == 0 {
                self.header.seq = seq.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst) as u32;
            }
            if self.header.stamp.nanos() == 0 {
                self.header.stamp = clock.now();
            }
        }
    }
    impl rosrust::rosmsg::RosMsg for OdasSstArrayStamped {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.header.encode(w.by_ref())?;
            rosrust::rosmsg::encode_variable_slice(&self.sources, w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                header: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                sources: rosrust::rosmsg::decode_variable_vec(r.by_ref())?,
            })
        }
    }
}
pub mod std_msgs {
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct Header {
        pub seq: u32,
        pub stamp: rosrust::Time,
        pub frame_id: ::std::string::String,
    }
    impl Header {}

    impl std::convert::From<Header> for rosrust::MsgValue {
        fn from(src: Header) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<Header> for rosrust::MsgMessage {
        fn from(src: Header) -> Self {
            let mut output = Self::new();
            output.insert("seq".into(), src.seq.into());
            output.insert("stamp".into(), src.stamp.into());
            output.insert("frame_id".into(), src.frame_id.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for Header {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for Header {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                seq: src.remove("seq").ok_or(())?.try_into()?,
                stamp: src.remove("stamp").ok_or(())?.try_into()?,
                frame_id: src.remove("frame_id").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for Header {
        fn eq(&self, other: &Self) -> bool {
            true && self.seq == other.seq
                && self.stamp == other.stamp
                && self.frame_id == other.frame_id
        }
    }
    impl std::fmt::Debug for Header {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(Header))
                .field(stringify!(seq), &self.seq)
                .field(stringify!(stamp), &self.stamp)
                .field(stringify!(frame_id), &self.frame_id)
                .finish()
        }
    }
    impl Default for Header {
        fn default() -> Self {
            Self {
                seq: Default::default(),
                stamp: Default::default(),
                frame_id: Default::default(),
            }
        }
    }
    impl rosrust::Message for Header {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "# Standard metadata for higher-level stamped data types.\n# This is generally used to \
             communicate timestamped data \n# in a particular coordinate frame.\n# \n# sequence \
             ID: consecutively increasing ID \nuint32 seq\n#Two-integer timestamp that is \
             expressed as:\n# * stamp.sec: seconds (stamp_secs) since epoch (in Python the \
             variable is called 'secs')\n# * stamp.nsec: nanoseconds since stamp_secs (in Python \
             the variable is called 'nsecs')\n# time-handling sugar is provided by the client \
             library\ntime stamp\n#Frame this data is associated with\nstring frame_id\n"
                .into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "2176decaecbce78abc3b96ef049fabed".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "std_msgs/Header".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for Header {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.seq.encode(w.by_ref())?;
            self.stamp.encode(w.by_ref())?;
            self.frame_id.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                seq: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                stamp: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                frame_id: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ColorRGBA {
        pub r: f32,
        pub g: f32,
        pub b: f32,
        pub a: f32,
    }
    impl ColorRGBA {}

    impl std::convert::From<ColorRGBA> for rosrust::MsgValue {
        fn from(src: ColorRGBA) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<ColorRGBA> for rosrust::MsgMessage {
        fn from(src: ColorRGBA) -> Self {
            let mut output = Self::new();
            output.insert("r".into(), src.r.into());
            output.insert("g".into(), src.g.into());
            output.insert("b".into(), src.b.into());
            output.insert("a".into(), src.a.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for ColorRGBA {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for ColorRGBA {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                r: src.remove("r").ok_or(())?.try_into()?,
                g: src.remove("g").ok_or(())?.try_into()?,
                b: src.remove("b").ok_or(())?.try_into()?,
                a: src.remove("a").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for ColorRGBA {
        fn eq(&self, other: &Self) -> bool {
            true && self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
        }
    }
    impl std::fmt::Debug for ColorRGBA {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(ColorRGBA))
                .field(stringify!(r), &self.r)
                .field(stringify!(g), &self.g)
                .field(stringify!(b), &self.b)
                .field(stringify!(a), &self.a)
                .finish()
        }
    }
    impl Default for ColorRGBA {
        fn default() -> Self {
            Self {
                r: Default::default(),
                g: Default::default(),
                b: Default::default(),
                a: Default::default(),
            }
        }
    }
    impl rosrust::Message for ColorRGBA {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "float32 r\nfloat32 g\nfloat32 b\nfloat32 a\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "a29a96539573343b1310c73607334b00".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "std_msgs/ColorRGBA".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for ColorRGBA {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.r.encode(w.by_ref())?;
            self.g.encode(w.by_ref())?;
            self.b.encode(w.by_ref())?;
            self.a.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                r: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                g: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                b: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                a: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
}
pub mod audio_common_msgs {
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct AudioDataStamped {
        pub header: super::std_msgs::Header,
        pub audio: super::audio_common_msgs::AudioData,
    }
    impl AudioDataStamped {}

    impl std::convert::From<AudioDataStamped> for rosrust::MsgValue {
        fn from(src: AudioDataStamped) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<AudioDataStamped> for rosrust::MsgMessage {
        fn from(src: AudioDataStamped) -> Self {
            let mut output = Self::new();
            output.insert("header".into(), src.header.into());
            output.insert("audio".into(), src.audio.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for AudioDataStamped {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for AudioDataStamped {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                header: src.remove("header").ok_or(())?.try_into()?,
                audio: src.remove("audio").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for AudioDataStamped {
        fn eq(&self, other: &Self) -> bool {
            true && self.header == other.header && self.audio == other.audio
        }
    }
    impl std::fmt::Debug for AudioDataStamped {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(AudioDataStamped))
                .field(stringify!(header), &self.header)
                .field(stringify!(audio), &self.audio)
                .finish()
        }
    }
    impl Default for AudioDataStamped {
        fn default() -> Self {
            Self {
                header: Default::default(),
                audio: Default::default(),
            }
        }
    }
    impl rosrust::Message for AudioDataStamped {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "std_msgs/Header header\naudio_common_msgs/AudioData audio\n\n================================================================================\nMSG: std_msgs/Header\n# Standard metadata for higher-level stamped data types.\n# This is generally used to communicate timestamped data \n# in a particular coordinate frame.\n# \n# sequence ID: consecutively increasing ID \nuint32 seq\n#Two-integer timestamp that is expressed as:\n# * stamp.sec: seconds (stamp_secs) since epoch (in Python the variable is called 'secs')\n# * stamp.nsec: nanoseconds since stamp_secs (in Python the variable is called 'nsecs')\n# time-handling sugar is provided by the client library\ntime stamp\n#Frame this data is associated with\nstring frame_id\n\n================================================================================\nMSG: audio_common_msgs/AudioData\nuint8[] data\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "3cdd84a06846af0dca4d0434908f9d96".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "audio_common_msgs/AudioDataStamped".into()
        }

        fn set_header(
            &mut self,
            clock: &::std::sync::Arc<dyn rosrust::Clock>,
            seq: &::std::sync::Arc<::std::sync::atomic::AtomicUsize>,
        ) {
            if self.header.seq == 0 {
                self.header.seq = seq.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst) as u32;
            }
            if self.header.stamp.nanos() == 0 {
                self.header.stamp = clock.now();
            }
        }
    }
    impl rosrust::rosmsg::RosMsg for AudioDataStamped {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.header.encode(w.by_ref())?;
            self.audio.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                header: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                audio: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct AudioData {
        pub data: Vec<u8>,
    }
    impl AudioData {}

    impl std::convert::From<AudioData> for rosrust::MsgValue {
        fn from(src: AudioData) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<AudioData> for rosrust::MsgMessage {
        fn from(src: AudioData) -> Self {
            let mut output = Self::new();
            output.insert("data".into(), src.data.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for AudioData {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for AudioData {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                data: src.remove("data").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for AudioData {
        fn eq(&self, other: &Self) -> bool {
            true && self.data == other.data
        }
    }
    impl std::fmt::Debug for AudioData {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(AudioData))
                .field(stringify!(data), &self.data)
                .finish()
        }
    }
    impl Default for AudioData {
        fn default() -> Self {
            Self {
                data: Default::default(),
            }
        }
    }
    impl rosrust::Message for AudioData {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "uint8[] data\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "f43a8e1b362b75baa741461b46adc7e0".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "audio_common_msgs/AudioData".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for AudioData {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            rosrust::rosmsg::encode_variable_primitive_slice(&self.data, w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                data: rosrust::rosmsg::decode_variable_primitive_vec(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct AudioInfo {
        pub channels: u8,
        pub sample_rate: u32,
        pub sample_format: ::std::string::String,
        pub bitrate: u32,
        pub coding_format: ::std::string::String,
    }
    impl AudioInfo {}

    impl std::convert::From<AudioInfo> for rosrust::MsgValue {
        fn from(src: AudioInfo) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<AudioInfo> for rosrust::MsgMessage {
        fn from(src: AudioInfo) -> Self {
            let mut output = Self::new();
            output.insert("channels".into(), src.channels.into());
            output.insert("sample_rate".into(), src.sample_rate.into());
            output.insert("sample_format".into(), src.sample_format.into());
            output.insert("bitrate".into(), src.bitrate.into());
            output.insert("coding_format".into(), src.coding_format.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for AudioInfo {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for AudioInfo {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                channels: src.remove("channels").ok_or(())?.try_into()?,
                sample_rate: src.remove("sample_rate").ok_or(())?.try_into()?,
                sample_format: src.remove("sample_format").ok_or(())?.try_into()?,
                bitrate: src.remove("bitrate").ok_or(())?.try_into()?,
                coding_format: src.remove("coding_format").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for AudioInfo {
        fn eq(&self, other: &Self) -> bool {
            true && self.channels == other.channels
                && self.sample_rate == other.sample_rate
                && self.sample_format == other.sample_format
                && self.bitrate == other.bitrate
                && self.coding_format == other.coding_format
        }
    }
    impl std::fmt::Debug for AudioInfo {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(AudioInfo))
                .field(stringify!(channels), &self.channels)
                .field(stringify!(sample_rate), &self.sample_rate)
                .field(stringify!(sample_format), &self.sample_format)
                .field(stringify!(bitrate), &self.bitrate)
                .field(stringify!(coding_format), &self.coding_format)
                .finish()
        }
    }
    impl Default for AudioInfo {
        fn default() -> Self {
            Self {
                channels: Default::default(),
                sample_rate: Default::default(),
                sample_format: Default::default(),
                bitrate: Default::default(),
                coding_format: Default::default(),
            }
        }
    }
    impl rosrust::Message for AudioInfo {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "# This message contains the audio meta data\n\n# Number of channels\nuint8 \
             channels\n# Sampling rate [Hz]\nuint32 sample_rate\n# Audio format (e.g. \
             S16LE)\nstring sample_format\n# Amount of audio data per second [bits/s]\nuint32 \
             bitrate\n# Audio coding format (e.g. WAVE, MP3)\nstring coding_format\n"
                .into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "9413d9b7029680d3b1db6ed0ae535f88".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "audio_common_msgs/AudioInfo".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for AudioInfo {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.channels.encode(w.by_ref())?;
            self.sample_rate.encode(w.by_ref())?;
            self.sample_format.encode(w.by_ref())?;
            self.bitrate.encode(w.by_ref())?;
            self.coding_format.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                channels: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                sample_rate: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                sample_format: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                bitrate: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                coding_format: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
}
pub mod visualization_msgs {
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct Marker {
        pub header: super::std_msgs::Header,
        pub ns: ::std::string::String,
        pub id: i32,
        pub type_: i32,
        pub action: i32,
        pub pose: super::geometry_msgs::Pose,
        pub scale: super::geometry_msgs::Vector3,
        pub color: super::std_msgs::ColorRGBA,
        pub lifetime: rosrust::Duration,
        pub frame_locked: bool,
        pub points: Vec<super::geometry_msgs::Point>,
        pub colors: Vec<super::std_msgs::ColorRGBA>,
        pub text: ::std::string::String,
        pub mesh_resource: ::std::string::String,
        pub mesh_use_embedded_materials: bool,
    }
    impl Marker {
        #[allow(dead_code, non_upper_case_globals)]
        pub const ADD: u8 = 0u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const ARROW: u8 = 0u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const CUBE: u8 = 1u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const CUBE_LIST: u8 = 6u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const CYLINDER: u8 = 3u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const DELETE: u8 = 2u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const DELETEALL: u8 = 3u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const LINE_LIST: u8 = 5u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const LINE_STRIP: u8 = 4u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const MESH_RESOURCE: u8 = 10u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const MODIFY: u8 = 0u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const POINTS: u8 = 8u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const SPHERE: u8 = 2u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const SPHERE_LIST: u8 = 7u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const TEXT_VIEW_FACING: u8 = 9u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const TRIANGLE_LIST: u8 = 11u8 as u8;
    }
    impl std::convert::From<Marker> for rosrust::MsgValue {
        fn from(src: Marker) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<Marker> for rosrust::MsgMessage {
        fn from(src: Marker) -> Self {
            let mut output = Self::new();
            output.insert("header".into(), src.header.into());
            output.insert("ns".into(), src.ns.into());
            output.insert("id".into(), src.id.into());
            output.insert("type".into(), src.type_.into());
            output.insert("action".into(), src.action.into());
            output.insert("pose".into(), src.pose.into());
            output.insert("scale".into(), src.scale.into());
            output.insert("color".into(), src.color.into());
            output.insert("lifetime".into(), src.lifetime.into());
            output.insert("frame_locked".into(), src.frame_locked.into());
            output.insert("points".into(), src.points.into());
            output.insert("colors".into(), src.colors.into());
            output.insert("text".into(), src.text.into());
            output.insert("mesh_resource".into(), src.mesh_resource.into());
            output.insert(
                "mesh_use_embedded_materials".into(),
                src.mesh_use_embedded_materials.into(),
            );
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for Marker {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for Marker {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                header: src.remove("header").ok_or(())?.try_into()?,
                ns: src.remove("ns").ok_or(())?.try_into()?,
                id: src.remove("id").ok_or(())?.try_into()?,
                type_: src.remove("type").ok_or(())?.try_into()?,
                action: src.remove("action").ok_or(())?.try_into()?,
                pose: src.remove("pose").ok_or(())?.try_into()?,
                scale: src.remove("scale").ok_or(())?.try_into()?,
                color: src.remove("color").ok_or(())?.try_into()?,
                lifetime: src.remove("lifetime").ok_or(())?.try_into()?,
                frame_locked: src.remove("frame_locked").ok_or(())?.try_into()?,
                points: src.remove("points").ok_or(())?.try_into()?,
                colors: src.remove("colors").ok_or(())?.try_into()?,
                text: src.remove("text").ok_or(())?.try_into()?,
                mesh_resource: src.remove("mesh_resource").ok_or(())?.try_into()?,
                mesh_use_embedded_materials: src
                    .remove("mesh_use_embedded_materials")
                    .ok_or(())?
                    .try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for Marker {
        fn eq(&self, other: &Self) -> bool {
            true && self.header == other.header
                && self.ns == other.ns
                && self.id == other.id
                && self.type_ == other.type_
                && self.action == other.action
                && self.pose == other.pose
                && self.scale == other.scale
                && self.color == other.color
                && self.lifetime == other.lifetime
                && self.frame_locked == other.frame_locked
                && self.points == other.points
                && self.colors == other.colors
                && self.text == other.text
                && self.mesh_resource == other.mesh_resource
                && self.mesh_use_embedded_materials == other.mesh_use_embedded_materials
        }
    }
    impl std::fmt::Debug for Marker {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(Marker))
                .field(stringify!(header), &self.header)
                .field(stringify!(ns), &self.ns)
                .field(stringify!(id), &self.id)
                .field(stringify!(type_), &self.type_)
                .field(stringify!(action), &self.action)
                .field(stringify!(pose), &self.pose)
                .field(stringify!(scale), &self.scale)
                .field(stringify!(color), &self.color)
                .field(stringify!(lifetime), &self.lifetime)
                .field(stringify!(frame_locked), &self.frame_locked)
                .field(stringify!(points), &self.points)
                .field(stringify!(colors), &self.colors)
                .field(stringify!(text), &self.text)
                .field(stringify!(mesh_resource), &self.mesh_resource)
                .field(
                    stringify!(mesh_use_embedded_materials),
                    &self.mesh_use_embedded_materials,
                )
                .finish()
        }
    }
    impl Default for Marker {
        fn default() -> Self {
            Self {
                header: Default::default(),
                ns: Default::default(),
                id: Default::default(),
                type_: Default::default(),
                action: Default::default(),
                pose: Default::default(),
                scale: Default::default(),
                color: Default::default(),
                lifetime: Default::default(),
                frame_locked: Default::default(),
                points: Default::default(),
                colors: Default::default(),
                text: Default::default(),
                mesh_resource: Default::default(),
                mesh_use_embedded_materials: Default::default(),
            }
        }
    }
    impl rosrust::Message for Marker {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "# See http://www.ros.org/wiki/rviz/DisplayTypes/Marker and http://www.ros.org/wiki/rviz/Tutorials/Markers%3A%20Basic%20Shapes for more information on using this message with rviz\n\nuint8 ARROW=0\nuint8 CUBE=1\nuint8 SPHERE=2\nuint8 CYLINDER=3\nuint8 LINE_STRIP=4\nuint8 LINE_LIST=5\nuint8 CUBE_LIST=6\nuint8 SPHERE_LIST=7\nuint8 POINTS=8\nuint8 TEXT_VIEW_FACING=9\nuint8 MESH_RESOURCE=10\nuint8 TRIANGLE_LIST=11\n\nuint8 ADD=0\nuint8 MODIFY=0\nuint8 DELETE=2\nuint8 DELETEALL=3\n\nHeader header                        # header for time/frame information\nstring ns                            # Namespace to place this object in... used in conjunction with id to create a unique name for the object\nint32 id \t\t                         # object ID useful in conjunction with the namespace for manipulating and deleting the object later\nint32 type \t\t                       # Type of object\nint32 action \t                       # 0 add/modify an object, 1 (deprecated), 2 deletes an object, 3 deletes all objects\ngeometry_msgs/Pose pose                 # Pose of the object\ngeometry_msgs/Vector3 scale             # Scale of the object 1,1,1 means default (usually 1 meter square)\nstd_msgs/ColorRGBA color             # Color [0.0-1.0]\nduration lifetime                    # How long the object should last before being automatically deleted.  0 means forever\nbool frame_locked                    # If this marker should be frame-locked, i.e. retransformed into its frame every timestep\n\n#Only used if the type specified has some use for them (eg. POINTS, LINE_STRIP, ...)\ngeometry_msgs/Point[] points\n#Only used if the type specified has some use for them (eg. POINTS, LINE_STRIP, ...)\n#number of colors must either be 0 or equal to the number of points\n#NOTE: alpha is not yet used\nstd_msgs/ColorRGBA[] colors\n\n# NOTE: only used for text markers\nstring text\n\n# NOTE: only used for MESH_RESOURCE markers\nstring mesh_resource\nbool mesh_use_embedded_materials\n\n================================================================================\nMSG: std_msgs/Header\n# Standard metadata for higher-level stamped data types.\n# This is generally used to communicate timestamped data \n# in a particular coordinate frame.\n# \n# sequence ID: consecutively increasing ID \nuint32 seq\n#Two-integer timestamp that is expressed as:\n# * stamp.sec: seconds (stamp_secs) since epoch (in Python the variable is called 'secs')\n# * stamp.nsec: nanoseconds since stamp_secs (in Python the variable is called 'nsecs')\n# time-handling sugar is provided by the client library\ntime stamp\n#Frame this data is associated with\nstring frame_id\n\n================================================================================\nMSG: geometry_msgs/Pose\n# A representation of pose in free space, composed of position and orientation. \nPoint position\nQuaternion orientation\n\n================================================================================\nMSG: geometry_msgs/Vector3\n# This represents a vector in free space. \n# It is only meant to represent a direction. Therefore, it does not\n# make sense to apply a translation to it (e.g., when applying a \n# generic rigid transformation to a Vector3, tf2 will only apply the\n# rotation). If you want your data to be translatable too, use the\n# geometry_msgs/Point message instead.\n\nfloat64 x\nfloat64 y\nfloat64 z\n\n================================================================================\nMSG: std_msgs/ColorRGBA\nfloat32 r\nfloat32 g\nfloat32 b\nfloat32 a\n\n================================================================================\nMSG: geometry_msgs/Point\n# This contains the position of a point in free space\nfloat64 x\nfloat64 y\nfloat64 z\n\n================================================================================\nMSG: geometry_msgs/Quaternion\n# This represents an orientation in free space in quaternion form.\n\nfloat64 x\nfloat64 y\nfloat64 z\nfloat64 w\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "4048c9de2a16f4ae8e0538085ebf1b97".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "visualization_msgs/Marker".into()
        }

        fn set_header(
            &mut self,
            clock: &::std::sync::Arc<dyn rosrust::Clock>,
            seq: &::std::sync::Arc<::std::sync::atomic::AtomicUsize>,
        ) {
            if self.header.seq == 0 {
                self.header.seq = seq.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst) as u32;
            }
            if self.header.stamp.nanos() == 0 {
                self.header.stamp = clock.now();
            }
        }
    }
    impl rosrust::rosmsg::RosMsg for Marker {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.header.encode(w.by_ref())?;
            self.ns.encode(w.by_ref())?;
            self.id.encode(w.by_ref())?;
            self.type_.encode(w.by_ref())?;
            self.action.encode(w.by_ref())?;
            self.pose.encode(w.by_ref())?;
            self.scale.encode(w.by_ref())?;
            self.color.encode(w.by_ref())?;
            self.lifetime.encode(w.by_ref())?;
            self.frame_locked.encode(w.by_ref())?;
            rosrust::rosmsg::encode_variable_slice(&self.points, w.by_ref())?;
            rosrust::rosmsg::encode_variable_slice(&self.colors, w.by_ref())?;
            self.text.encode(w.by_ref())?;
            self.mesh_resource.encode(w.by_ref())?;
            self.mesh_use_embedded_materials.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                header: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                ns: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                id: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                type_: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                action: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                pose: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                scale: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                color: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                lifetime: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                frame_locked: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                points: rosrust::rosmsg::decode_variable_vec(r.by_ref())?,
                colors: rosrust::rosmsg::decode_variable_vec(r.by_ref())?,
                text: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                mesh_resource: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                mesh_use_embedded_materials: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
}
pub mod sensor_msgs {
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct CompressedImage {
        pub header: super::std_msgs::Header,
        pub format: ::std::string::String,
        pub data: Vec<u8>,
    }
    impl CompressedImage {}

    impl std::convert::From<CompressedImage> for rosrust::MsgValue {
        fn from(src: CompressedImage) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<CompressedImage> for rosrust::MsgMessage {
        fn from(src: CompressedImage) -> Self {
            let mut output = Self::new();
            output.insert("header".into(), src.header.into());
            output.insert("format".into(), src.format.into());
            output.insert("data".into(), src.data.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for CompressedImage {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for CompressedImage {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                header: src.remove("header").ok_or(())?.try_into()?,
                format: src.remove("format").ok_or(())?.try_into()?,
                data: src.remove("data").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for CompressedImage {
        fn eq(&self, other: &Self) -> bool {
            true && self.header == other.header
                && self.format == other.format
                && self.data == other.data
        }
    }
    impl std::fmt::Debug for CompressedImage {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(CompressedImage))
                .field(stringify!(header), &self.header)
                .field(stringify!(format), &self.format)
                .field(stringify!(data), &self.data)
                .finish()
        }
    }
    impl Default for CompressedImage {
        fn default() -> Self {
            Self {
                header: Default::default(),
                format: Default::default(),
                data: Default::default(),
            }
        }
    }
    impl rosrust::Message for CompressedImage {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "# This message contains a compressed image\n\nHeader header        # Header timestamp should be acquisition time of image\n                     # Header frame_id should be optical frame of camera\n                     # origin of frame should be optical center of camera\n                     # +x should point to the right in the image\n                     # +y should point down in the image\n                     # +z should point into to plane of the image\n\nstring format        # Specifies the format of the data\n                     #   Acceptable values:\n                     #     jpeg, png\nuint8[] data         # Compressed image buffer\n\n================================================================================\nMSG: std_msgs/Header\n# Standard metadata for higher-level stamped data types.\n# This is generally used to communicate timestamped data \n# in a particular coordinate frame.\n# \n# sequence ID: consecutively increasing ID \nuint32 seq\n#Two-integer timestamp that is expressed as:\n# * stamp.sec: seconds (stamp_secs) since epoch (in Python the variable is called 'secs')\n# * stamp.nsec: nanoseconds since stamp_secs (in Python the variable is called 'nsecs')\n# time-handling sugar is provided by the client library\ntime stamp\n#Frame this data is associated with\nstring frame_id\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "8f7a12909da2c9d3332d540a0977563f".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "sensor_msgs/CompressedImage".into()
        }

        fn set_header(
            &mut self,
            clock: &::std::sync::Arc<dyn rosrust::Clock>,
            seq: &::std::sync::Arc<::std::sync::atomic::AtomicUsize>,
        ) {
            if self.header.seq == 0 {
                self.header.seq = seq.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst) as u32;
            }
            if self.header.stamp.nanos() == 0 {
                self.header.stamp = clock.now();
            }
        }
    }
    impl rosrust::rosmsg::RosMsg for CompressedImage {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.header.encode(w.by_ref())?;
            self.format.encode(w.by_ref())?;
            rosrust::rosmsg::encode_variable_primitive_slice(&self.data, w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                header: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                format: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                data: rosrust::rosmsg::decode_variable_primitive_vec(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct PointField {
        pub name: ::std::string::String,
        pub offset: u32,
        pub datatype: u8,
        pub count: u32,
    }
    impl PointField {
        #[allow(dead_code, non_upper_case_globals)]
        pub const FLOAT32: u8 = 7u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const FLOAT64: u8 = 8u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const INT16: u8 = 3u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const INT32: u8 = 5u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const INT8: u8 = 1u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const UINT16: u8 = 4u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const UINT32: u8 = 6u8 as u8;
        #[allow(dead_code, non_upper_case_globals)]
        pub const UINT8: u8 = 2u8 as u8;
    }
    impl std::convert::From<PointField> for rosrust::MsgValue {
        fn from(src: PointField) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<PointField> for rosrust::MsgMessage {
        fn from(src: PointField) -> Self {
            let mut output = Self::new();
            output.insert("name".into(), src.name.into());
            output.insert("offset".into(), src.offset.into());
            output.insert("datatype".into(), src.datatype.into());
            output.insert("count".into(), src.count.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for PointField {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for PointField {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                name: src.remove("name").ok_or(())?.try_into()?,
                offset: src.remove("offset").ok_or(())?.try_into()?,
                datatype: src.remove("datatype").ok_or(())?.try_into()?,
                count: src.remove("count").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for PointField {
        fn eq(&self, other: &Self) -> bool {
            true && self.name == other.name
                && self.offset == other.offset
                && self.datatype == other.datatype
                && self.count == other.count
        }
    }
    impl std::fmt::Debug for PointField {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(PointField))
                .field(stringify!(name), &self.name)
                .field(stringify!(offset), &self.offset)
                .field(stringify!(datatype), &self.datatype)
                .field(stringify!(count), &self.count)
                .finish()
        }
    }
    impl Default for PointField {
        fn default() -> Self {
            Self {
                name: Default::default(),
                offset: Default::default(),
                datatype: Default::default(),
                count: Default::default(),
            }
        }
    }
    impl rosrust::Message for PointField {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "# This message holds the description of one point entry in the\n# PointCloud2 message \
             format.\nuint8 INT8    = 1\nuint8 UINT8   = 2\nuint8 INT16   = 3\nuint8 UINT16  = \
             4\nuint8 INT32   = 5\nuint8 UINT32  = 6\nuint8 FLOAT32 = 7\nuint8 FLOAT64 = \
             8\n\nstring name      # Name of field\nuint32 offset    # Offset from start of point \
             struct\nuint8  datatype  # Datatype enumeration, see above\nuint32 count     # How \
             many elements in the field\n"
                .into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "268eacb2962780ceac86cbd17e328150".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "sensor_msgs/PointField".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for PointField {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.name.encode(w.by_ref())?;
            self.offset.encode(w.by_ref())?;
            self.datatype.encode(w.by_ref())?;
            self.count.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                name: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                offset: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                datatype: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                count: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct PointCloud2 {
        pub header: super::std_msgs::Header,
        pub height: u32,
        pub width: u32,
        pub fields: Vec<PointField>,
        pub is_bigendian: bool,
        pub point_step: u32,
        pub row_step: u32,
        pub data: Vec<u8>,
        pub is_dense: bool,
    }
    impl PointCloud2 {}

    impl std::convert::From<PointCloud2> for rosrust::MsgValue {
        fn from(src: PointCloud2) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<PointCloud2> for rosrust::MsgMessage {
        fn from(src: PointCloud2) -> Self {
            let mut output = Self::new();
            output.insert("header".into(), src.header.into());
            output.insert("height".into(), src.height.into());
            output.insert("width".into(), src.width.into());
            output.insert("fields".into(), src.fields.into());
            output.insert("is_bigendian".into(), src.is_bigendian.into());
            output.insert("point_step".into(), src.point_step.into());
            output.insert("row_step".into(), src.row_step.into());
            output.insert("data".into(), src.data.into());
            output.insert("is_dense".into(), src.is_dense.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for PointCloud2 {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for PointCloud2 {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                header: src.remove("header").ok_or(())?.try_into()?,
                height: src.remove("height").ok_or(())?.try_into()?,
                width: src.remove("width").ok_or(())?.try_into()?,
                fields: src.remove("fields").ok_or(())?.try_into()?,
                is_bigendian: src.remove("is_bigendian").ok_or(())?.try_into()?,
                point_step: src.remove("point_step").ok_or(())?.try_into()?,
                row_step: src.remove("row_step").ok_or(())?.try_into()?,
                data: src.remove("data").ok_or(())?.try_into()?,
                is_dense: src.remove("is_dense").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for PointCloud2 {
        fn eq(&self, other: &Self) -> bool {
            true && self.header == other.header
                && self.height == other.height
                && self.width == other.width
                && self.fields == other.fields
                && self.is_bigendian == other.is_bigendian
                && self.point_step == other.point_step
                && self.row_step == other.row_step
                && self.data == other.data
                && self.is_dense == other.is_dense
        }
    }
    impl std::fmt::Debug for PointCloud2 {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(PointCloud2))
                .field(stringify!(header), &self.header)
                .field(stringify!(height), &self.height)
                .field(stringify!(width), &self.width)
                .field(stringify!(fields), &self.fields)
                .field(stringify!(is_bigendian), &self.is_bigendian)
                .field(stringify!(point_step), &self.point_step)
                .field(stringify!(row_step), &self.row_step)
                .field(stringify!(data), &self.data)
                .field(stringify!(is_dense), &self.is_dense)
                .finish()
        }
    }
    impl Default for PointCloud2 {
        fn default() -> Self {
            Self {
                header: Default::default(),
                height: Default::default(),
                width: Default::default(),
                fields: Default::default(),
                is_bigendian: Default::default(),
                point_step: Default::default(),
                row_step: Default::default(),
                data: Default::default(),
                is_dense: Default::default(),
            }
        }
    }
    impl rosrust::Message for PointCloud2 {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "# This message holds a collection of N-dimensional points, which may\n# contain additional information such as normals, intensity, etc. The\n# point data is stored as a binary blob, its layout described by the\n# contents of the \"fields\" array.\n\n# The point cloud data may be organized 2d (image-like) or 1d\n# (unordered). Point clouds organized as 2d images may be produced by\n# camera depth sensors such as stereo or time-of-flight.\n\n# Time of sensor data acquisition, and the coordinate frame ID (for 3d\n# points).\nHeader header\n\n# 2D structure of the point cloud. If the cloud is unordered, height is\n# 1 and width is the length of the point cloud.\nuint32 height\nuint32 width\n\n# Describes the channels and their layout in the binary data blob.\nPointField[] fields\n\nbool    is_bigendian # Is this data bigendian?\nuint32  point_step   # Length of a point in bytes\nuint32  row_step     # Length of a row in bytes\nuint8[] data         # Actual point data, size is (row_step*height)\n\nbool is_dense        # True if there are no invalid points\n\n================================================================================\nMSG: std_msgs/Header\n# Standard metadata for higher-level stamped data types.\n# This is generally used to communicate timestamped data \n# in a particular coordinate frame.\n# \n# sequence ID: consecutively increasing ID \nuint32 seq\n#Two-integer timestamp that is expressed as:\n# * stamp.sec: seconds (stamp_secs) since epoch (in Python the variable is called 'secs')\n# * stamp.nsec: nanoseconds since stamp_secs (in Python the variable is called 'nsecs')\n# time-handling sugar is provided by the client library\ntime stamp\n#Frame this data is associated with\nstring frame_id\n\n================================================================================\nMSG: sensor_msgs/PointField\n# This message holds the description of one point entry in the\n# PointCloud2 message format.\nuint8 INT8    = 1\nuint8 UINT8   = 2\nuint8 INT16   = 3\nuint8 UINT16  = 4\nuint8 INT32   = 5\nuint8 UINT32  = 6\nuint8 FLOAT32 = 7\nuint8 FLOAT64 = 8\n\nstring name      # Name of field\nuint32 offset    # Offset from start of point struct\nuint8  datatype  # Datatype enumeration, see above\nuint32 count     # How many elements in the field\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "1158d486dd51d683ce2f1be655c3c181".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "sensor_msgs/PointCloud2".into()
        }

        fn set_header(
            &mut self,
            clock: &::std::sync::Arc<dyn rosrust::Clock>,
            seq: &::std::sync::Arc<::std::sync::atomic::AtomicUsize>,
        ) {
            if self.header.seq == 0 {
                self.header.seq = seq.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst) as u32;
            }
            if self.header.stamp.nanos() == 0 {
                self.header.stamp = clock.now();
            }
        }
    }
    impl rosrust::rosmsg::RosMsg for PointCloud2 {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.header.encode(w.by_ref())?;
            self.height.encode(w.by_ref())?;
            self.width.encode(w.by_ref())?;
            rosrust::rosmsg::encode_variable_slice(&self.fields, w.by_ref())?;
            self.is_bigendian.encode(w.by_ref())?;
            self.point_step.encode(w.by_ref())?;
            self.row_step.encode(w.by_ref())?;
            rosrust::rosmsg::encode_variable_primitive_slice(&self.data, w.by_ref())?;
            self.is_dense.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                header: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                height: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                width: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                fields: rosrust::rosmsg::decode_variable_vec(r.by_ref())?,
                is_bigendian: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                point_step: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                row_step: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                data: rosrust::rosmsg::decode_variable_primitive_vec(r.by_ref())?,
                is_dense: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
}
pub use audio_common_msgs::{AudioData, AudioDataStamped, AudioInfo};
pub use geometry_msgs::{Point, Pose, PoseArray, Quaternion, Vector3};
pub use odas_ros::{OdasSsl, OdasSslArrayStamped, OdasSst, OdasSstArrayStamped};
pub use sensor_msgs::{CompressedImage, PointCloud2, PointField};
pub use ssloc_ros_msgs::{Ssl, SslArray, SssMapping, Sst, SstArray};
pub use std_msgs::{ColorRGBA, Header};
pub use visualization_msgs::Marker;
