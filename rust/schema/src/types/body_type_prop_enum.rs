use crate::prelude::*;

use super::qualitative_value::QualitativeValue;
use super::text::Text;
use super::url::URL;


/// http://schema.org/bodyType
/// Indicates the design and body style of the vehicle (e.g. station wagon, hatchback, etc.).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum BodyTypePropEnum {
    QualitativeValue(QualitativeValue),
    Text(Text),
    URL(URL),
}
