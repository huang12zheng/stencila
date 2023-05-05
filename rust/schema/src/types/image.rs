use crate::prelude::*;

use super::image_object::ImageObject;
use super::url::URL;

/// An image of the item. This can be a <a class="localLink" href="/URL">URL</a> or a fully described <a class="localLink" href="/ImageObject">ImageObject</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum image {
    ImageObject(ImageObject),
    URL(URL),
}
