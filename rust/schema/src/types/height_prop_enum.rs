use crate::prelude::*;

use super::distance::Distance;
use super::quantitative_value::QuantitativeValue;


/// http://schema.org/height
/// The height of the item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum HeightPropEnum {
    Distance(Distance),
    QuantitativeValue(QuantitativeValue),
}
