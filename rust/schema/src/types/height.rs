use crate::prelude::*;

use super::distance::Distance;
use super::quantitative_value::QuantitativeValue;

/// The height of the item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum height {
    Distance(Distance),
    QuantitativeValue(QuantitativeValue),
}
