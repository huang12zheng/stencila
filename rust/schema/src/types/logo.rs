use crate::prelude::*;

use super::image_object::ImageObject;
use super::url::URL;

/// An associated logo.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum logo {
    ImageObject(ImageObject),
    URL(URL),
}
