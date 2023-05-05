// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;



/// https://schema.org/URL
/// * COMMENT:
/// Data type: URL.
/// * EXTEND FROM:
/// https://schema.org/Text
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct URL {
    
}

impl URL {
    pub fn new() -> Self {
        Self {
            
        }
    }
}
