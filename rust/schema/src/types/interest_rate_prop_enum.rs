use crate::prelude::*;

use super::number::Number;
use super::quantitative_value::QuantitativeValue;


/// http://schema.org/interestRate
/// The interest rate, charged or paid, applicable to the financial product. Note: This is different from the calculated annualPercentageRate.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum InterestRatePropEnum {
    Number(Number),
    QuantitativeValue(QuantitativeValue),
}
