use crate::prelude::*;

use super::image_object::ImageObject;
use super::photograph::Photograph;

/// A photograph of this place.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum photo {
    ImageObject(ImageObject),
    Photograph(Photograph),
}
