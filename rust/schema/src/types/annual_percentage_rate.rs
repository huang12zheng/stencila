use crate::prelude::*;

use super::number::Number;
use super::quantitative_value::QuantitativeValue;

/// The annual rate that is charged for borrowing (or made by investing), expressed as a single percentage number that represents the actual yearly cost of funds over the term of a loan. This includes any fees or additional costs associated with the transaction.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum annualPercentageRate {
    Number(Number),
    QuantitativeValue(QuantitativeValue),
}
