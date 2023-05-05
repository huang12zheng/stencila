use crate::prelude::*;

use super::energy::Energy;
use super::quantitative_value::QuantitativeValue;


/// http://schema.org/workload
/// Quantitative measure of the physiologic output of the exercise; also referred to as energy expenditure.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum WorkloadPropEnum {
    Energy(Energy),
    QuantitativeValue(QuantitativeValue),
}
