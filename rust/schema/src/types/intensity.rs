use crate::prelude::*;

use super::quantitative_value::QuantitativeValue;
use super::text::Text;

/// Quantitative measure gauging the degree of force involved in the exercise, for example, heartbeats per minute. May include the velocity of the movement.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum intensity {
    QuantitativeValue(QuantitativeValue),
    Text(Text),
}
