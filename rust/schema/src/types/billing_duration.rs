use crate::prelude::*;

use super::duration::Duration;
use super::number::Number;
use super::quantitative_value::QuantitativeValue;

/// Specifies for how long this price (or price component) will be billed. Can be used, for example, to model the contractual duration of a subscription or payment plan. Type can be either a Duration or a Number (in which case the unit of measurement, for example month, is specified by the unitCode property).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum billingDuration {
    Duration(Duration),
    Number(Number),
    QuantitativeValue(QuantitativeValue),
}
