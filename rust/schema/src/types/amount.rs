use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::number::Number;

/// The amount of money.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum amount {
    MonetaryAmount(MonetaryAmount),
    Number(Number),
}
