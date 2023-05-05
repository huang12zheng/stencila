// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;



/// https://schema.org/Integer
/// * COMMENT:
/// Data type: Integer.
/// * EXTEND FROM:
/// https://schema.org/Number
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Integer {
    
}

impl Integer {
    pub fn new() -> Self {
        Self {
            
        }
    }
}
