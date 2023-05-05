use crate::prelude::*;

use super::property_value::PropertyValue;
use super::text::Text;

/// exif data for this object.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum exifData {
    PropertyValue(PropertyValue),
    Text(Text),
}
