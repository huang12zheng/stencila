use crate::prelude::*;

use super::image_object::ImageObject;
use super::url::URL;


/// http://schema.org/screenshot
/// A link to a screenshot image of the app.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ScreenshotPropEnum {
    ImageObject(ImageObject),
    URL(URL),
}
