use crate::prelude::*;

use super::number::Number;
use super::quantitative_value::QuantitativeValue;

/// The interest rate, charged or paid, applicable to the financial product. Note: This is different from the calculated annualPercentageRate.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum interestRate {
    Number(Number),
    QuantitativeValue(QuantitativeValue),
}
