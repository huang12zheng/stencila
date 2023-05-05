use crate::prelude::*;

use super::property_value::PropertyValue;
use super::text::Text;


/// http://schema.org/exifData
/// exif data for this object.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ExifDataPropEnum {
    PropertyValue(PropertyValue),
    Text(Text),
}
