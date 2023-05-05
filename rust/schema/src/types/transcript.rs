use crate::prelude::*;

use super::audio_object::AudioObject;
use super::video_object::VideoObject;

/// If this MediaObject is an AudioObject or VideoObject, the transcript of that object.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum transcript {
    AudioObject(AudioObject),
    VideoObject(VideoObject),
}
