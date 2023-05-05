use crate::prelude::*;

use super::property_value::PropertyValue;
use super::text::Text;

/// The variableMeasured property can indicate (repeated as necessary) the  variables that are measured in some dataset, either described as text or as pairs of identifier and description using PropertyValue.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum variableMeasured {
    PropertyValue(PropertyValue),
    Text(Text),
}
