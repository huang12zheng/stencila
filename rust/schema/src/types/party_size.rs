use crate::prelude::*;

use super::integer::Integer;
use super::quantitative_value::QuantitativeValue;

/// Number of people the reservation should accommodate.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum partySize {
    Integer(Integer),
    QuantitativeValue(QuantitativeValue),
}
