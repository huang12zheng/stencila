use crate::prelude::*;

use super::image_object::ImageObject;
use super::photograph::Photograph;


/// http://schema.org/photo
/// A photograph of this place.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum PhotoPropEnum {
    ImageObject(ImageObject),
    Photograph(Photograph),
}
