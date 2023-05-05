use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::number::Number;


/// http://schema.org/monthlyMinimumRepaymentAmount
/// The minimum payment is the lowest amount of money that one is required to pay on a credit card statement each month.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum MonthlyMinimumRepaymentAmountPropEnum {
    MonetaryAmount(MonetaryAmount),
    Number(Number),
}
