use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::number::Number;

/// The minimum payment is the lowest amount of money that one is required to pay on a credit card statement each month.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum monthlyMinimumRepaymentAmount {
    MonetaryAmount(MonetaryAmount),
    Number(Number),
}
