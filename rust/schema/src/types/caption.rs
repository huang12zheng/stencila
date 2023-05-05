use crate::prelude::*;

use super::audio_object::AudioObject;
use super::image_object::ImageObject;
use super::video_object::VideoObject;

/// The caption for this object. For downloadable machine formats (closed caption, subtitles etc.) use MediaObject and indicate the <a class="localLink" href="/encodingFormat">encodingFormat</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum caption {
    AudioObject(AudioObject),
    ImageObject(ImageObject),
    VideoObject(VideoObject),
}
