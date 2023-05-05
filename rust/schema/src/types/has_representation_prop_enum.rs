use crate::prelude::*;

use super::property_value::PropertyValue;
use super::text::Text;
use super::url::URL;


/// http://schema.org/hasRepresentation
/// A common representation such as a protein sequence or chemical structure for this entity. For images use schema.org/image.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum HasRepresentationPropEnum {
    PropertyValue(PropertyValue),
    Text(Text),
    URL(URL),
}
