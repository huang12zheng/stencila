use crate::prelude::*;

use super::media_object::MediaObject;
use super::text::Text;


/// http://schema.org/caption
/// The caption for this object. For downloadable machine formats (closed caption, subtitles etc.) use MediaObject and indicate the <a class="localLink" href="/encodingFormat">encodingFormat</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CaptionPropEnum {
    MediaObject(MediaObject),
    Text(Text),
}
