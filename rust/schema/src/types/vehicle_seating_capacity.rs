use crate::prelude::*;

use super::number::Number;
use super::quantitative_value::QuantitativeValue;

/// The number of passengers that can be seated in the vehicle, both in terms of the physical space available, and in terms of limitations set by law.<br/><br/>  Typical unit code(s): C62 for persons.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum vehicleSeatingCapacity {
    Number(Number),
    QuantitativeValue(QuantitativeValue),
}
