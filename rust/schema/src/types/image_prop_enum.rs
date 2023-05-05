use crate::prelude::*;

use super::image_object::ImageObject;
use super::url::URL;


/// http://schema.org/image
/// An image of the item. This can be a <a class="localLink" href="/URL">URL</a> or a fully described <a class="localLink" href="/ImageObject">ImageObject</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ImagePropEnum {
    ImageObject(ImageObject),
    URL(URL),
}
