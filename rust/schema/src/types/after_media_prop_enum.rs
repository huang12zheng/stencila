use crate::prelude::*;

use super::media_object::MediaObject;
use super::url::URL;


/// http://schema.org/afterMedia
/// A media object representing the circumstances after performing this direction.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AfterMediaPropEnum {
    MediaObject(MediaObject),
    URL(URL),
}
