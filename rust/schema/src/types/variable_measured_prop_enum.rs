use crate::prelude::*;

use super::property_value::PropertyValue;
use super::text::Text;


/// http://schema.org/variableMeasured
/// The variableMeasured property can indicate (repeated as necessary) the  variables that are measured in some dataset, either described as text or as pairs of identifier and description using PropertyValue.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum VariableMeasuredPropEnum {
    PropertyValue(PropertyValue),
    Text(Text),
}
