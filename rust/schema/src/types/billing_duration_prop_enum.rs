use crate::prelude::*;

use super::duration::Duration;
use super::number::Number;
use super::quantitative_value::QuantitativeValue;


/// http://schema.org/billingDuration
/// Specifies for how long this price (or price component) will be billed. Can be used, for example, to model the contractual duration of a subscription or payment plan. Type can be either a Duration or a Number (in which case the unit of measurement, for example month, is specified by the unitCode property).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum BillingDurationPropEnum {
    Duration(Duration),
    Number(Number),
    QuantitativeValue(QuantitativeValue),
}
