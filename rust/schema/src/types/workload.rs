use crate::prelude::*;

use super::energy::Energy;
use super::quantitative_value::QuantitativeValue;

/// Quantitative measure of the physiologic output of the exercise; also referred to as energy expenditure.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum workload {
    Energy(Energy),
    QuantitativeValue(QuantitativeValue),
}
