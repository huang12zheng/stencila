use crate::prelude::*;

use super::number::Number;
use super::quantitative_value::QuantitativeValue;

/// The number of rooms (excluding bathrooms and closets) of the accommodation or lodging business. Typical unit code(s): ROM for room or C62 for no unit. The type of room can be put in the unitText property of the QuantitativeValue.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum numberOfRooms {
    Number(Number),
    QuantitativeValue(QuantitativeValue),
}
