// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;



/// https://schema.org/Date
/// * COMMENT:
/// A date value in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a>.
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Date {
    
}

impl Date {
    pub fn new() -> Self {
        Self {
            
        }
    }
}
