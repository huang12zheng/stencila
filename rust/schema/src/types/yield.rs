use crate::prelude::*;

use super::quantitative_value::QuantitativeValue;
use super::text::Text;

/// The quantity that results by performing instructions. For example, a paper airplane, 10 personalized candles.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum yield {
    QuantitativeValue(QuantitativeValue),
    Text(Text),
}
