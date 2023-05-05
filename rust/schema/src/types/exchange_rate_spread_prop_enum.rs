use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::number::Number;


/// http://schema.org/exchangeRateSpread
/// The difference between the price at which a broker or other intermediary buys and sells foreign currency.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ExchangeRateSpreadPropEnum {
    MonetaryAmount(MonetaryAmount),
    Number(Number),
}
