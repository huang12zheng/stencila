use crate::prelude::*;

use super::quantitative_value::QuantitativeValue;
use super::text::Text;

/// The quantity of the materials being described or an expression of the physical space they occupy.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum materialExtent {
    QuantitativeValue(QuantitativeValue),
    Text(Text),
}
