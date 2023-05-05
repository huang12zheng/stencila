use crate::prelude::*;

use super::media_object::MediaObject;
use super::url::URL;

/// A media object representing the circumstances after performing this direction.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum afterMedia {
    MediaObject(MediaObject),
    URL(URL),
}
