use crate::prelude::*;

use super::image_object::ImageObject;
use super::url::URL;

/// A link to a screenshot image of the app.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum screenshot {
    ImageObject(ImageObject),
    URL(URL),
}
