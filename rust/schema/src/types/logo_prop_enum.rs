use crate::prelude::*;

use super::image_object::ImageObject;
use super::url::URL;


/// http://schema.org/logo
/// An associated logo.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LogoPropEnum {
    ImageObject(ImageObject),
    URL(URL),
}
