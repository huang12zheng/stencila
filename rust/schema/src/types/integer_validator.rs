// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::number::Number;
use super::number_validator::NumberValidator;
use super::string::String;

/// A validator specifying the constraints on an integer node.
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct IntegerValidator {
    /// The type of this item
    pub r#type: MustBe!("IntegerValidator"),

    /// The identifier for this item
    pub id: Option<String>,

    /// The inclusive lower limit for a numeric node.
    pub minimum: Option<Number>,

    /// The exclusive lower limit for a numeric node.
    pub exclusive_minimum: Option<Number>,

    /// The inclusive upper limit for a numeric node.
    pub maximum: Option<Number>,

    /// The exclusive upper limit for a numeric node.
    pub exclusive_maximum: Option<Number>,

    /// A number that a numeric node must be a multiple of.
    pub multiple_of: Option<Number>,
}

impl IntegerValidator {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
impl_into!(IntegerValidator, NumberValidator);
impl_merge!(IntegerValidator, NumberValidator);
