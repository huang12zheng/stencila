use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::number::Number;

/// The difference between the price at which a broker or other intermediary buys and sells foreign currency.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum exchangeRateSpread {
    MonetaryAmount(MonetaryAmount),
    Number(Number),
}
