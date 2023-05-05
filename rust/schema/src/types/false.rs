// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;



/// https://schema.org/False
/// * COMMENT:
/// The boolean value false.
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct False {
    
}

impl False {
    pub fn new() -> Self {
        Self {
            
        }
    }
}
