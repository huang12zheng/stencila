use crate::prelude::*;

use super::clip::Clip;
use super::video_object::VideoObject;

/// An embedded video object.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum video {
    Clip(Clip),
    VideoObject(VideoObject),
}
