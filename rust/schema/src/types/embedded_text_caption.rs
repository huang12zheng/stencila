use crate::prelude::*;

use super::audio_object::AudioObject;
use super::image_object::ImageObject;
use super::video_object::VideoObject;

/// Represents textual captioning from a <a class="localLink" href="/MediaObject">MediaObject</a>, e.g. text of a 'meme'.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum embeddedTextCaption {
    AudioObject(AudioObject),
    ImageObject(ImageObject),
    VideoObject(VideoObject),
}
