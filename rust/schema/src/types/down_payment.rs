use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::number::Number;

/// a type of payment made in cash during the onset of the purchase of an expensive good/service. The payment typically represents only a percentage of the full purchase price.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum downPayment {
    MonetaryAmount(MonetaryAmount),
    Number(Number),
}
