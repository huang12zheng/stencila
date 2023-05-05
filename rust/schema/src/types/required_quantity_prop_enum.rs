use crate::prelude::*;

use super::number::Number;
use super::quantitative_value::QuantitativeValue;
use super::text::Text;


/// http://schema.org/requiredQuantity
/// The required quantity of the item(s).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum RequiredQuantityPropEnum {
    Number(Number),
    QuantitativeValue(QuantitativeValue),
    Text(Text),
}
