use crate::prelude::*;

use super::media_object::MediaObject;
use super::text::Text;

/// The caption for this object. For downloadable machine formats (closed caption, subtitles etc.) use MediaObject and indicate the <a class="localLink" href="/encodingFormat">encodingFormat</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum caption {
    MediaObject(MediaObject),
    Text(Text),
}
