use crate::prelude::*;

use super::property_value::PropertyValue;
use super::text::Text;
use super::url::URL;

/// A common representation such as a protein sequence or chemical structure for this entity. For images use schema.org/image.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum hasRepresentation {
    PropertyValue(PropertyValue),
    Text(Text),
    URL(URL),
}
