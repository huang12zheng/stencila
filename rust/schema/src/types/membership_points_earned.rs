use crate::prelude::*;

use super::number::Number;
use super::quantitative_value::QuantitativeValue;

/// The number of membership points earned by the member. If necessary, the unitText can be used to express the units the points are issued in. (E.g. stars, miles, etc.)
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum membershipPointsEarned {
    Number(Number),
    QuantitativeValue(QuantitativeValue),
}
