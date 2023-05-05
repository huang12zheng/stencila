use crate::prelude::*;

use super::image_object::ImageObject;
use super::url::URL;


/// http://schema.org/layoutImage
/// A schematic image showing the floorplan layout.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LayoutImagePropEnum {
    ImageObject(ImageObject),
    URL(URL),
}
