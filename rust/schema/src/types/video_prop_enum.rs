use crate::prelude::*;

use super::clip::Clip;
use super::video_object::VideoObject;


/// http://schema.org/video
/// An embedded video object.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum VideoPropEnum {
    Clip(Clip),
    VideoObject(VideoObject),
}
