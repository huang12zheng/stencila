use crate::prelude::*;

use super::quantitative_value::QuantitativeValue;
use super::text::Text;


/// http://schema.org/materialExtent
/// The quantity of the materials being described or an expression of the physical space they occupy.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum MaterialExtentPropEnum {
    QuantitativeValue(QuantitativeValue),
    Text(Text),
}
