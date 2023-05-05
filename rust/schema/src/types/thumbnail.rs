use crate::prelude::*;

use super::image_object::ImageObject;
use super::video_object::VideoObject;

/// Thumbnail image for an image or video.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum thumbnail {
    ImageObject(ImageObject),
    VideoObject(VideoObject),
}
