use crate::prelude::*;

use super::number::Number;
use super::quantitative_value::QuantitativeValue;
use super::text::Text;

/// The required quantity of the item(s).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum requiredQuantity {
    Number(Number),
    QuantitativeValue(QuantitativeValue),
    Text(Text),
}
