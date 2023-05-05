use crate::prelude::*;

use super::image_object::ImageObject;
use super::url::URL;

/// A schematic image showing the floorplan layout.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum layoutImage {
    ImageObject(ImageObject),
    URL(URL),
}
