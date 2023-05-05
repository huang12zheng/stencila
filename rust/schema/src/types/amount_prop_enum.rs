use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::number::Number;


/// http://schema.org/amount
/// The amount of money.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AmountPropEnum {
    MonetaryAmount(MonetaryAmount),
    Number(Number),
}
