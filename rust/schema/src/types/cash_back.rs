use crate::prelude::*;

use super::boolean::Boolean;
use super::number::Number;

/// A cardholder benefit that pays the cardholder a small percentage of their net expenditures.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum cashBack {
    Boolean(Boolean),
    Number(Number),
}
