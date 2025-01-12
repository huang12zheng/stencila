// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::date::Date;
use super::string::String;

/// A validator specifying the constraints on a date.
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct DateValidator {
    /// The type of this item
    pub r#type: MustBe!("DateValidator"),

    /// The identifier for this item
    pub id: Option<String>,

    /// The inclusive lower limit for a date.
    pub minimum: Option<Date>,

    /// The inclusive upper limit for a date.
    pub maximum: Option<Date>,
}

impl DateValidator {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
