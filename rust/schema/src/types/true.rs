// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;



/// https://schema.org/True
/// * COMMENT:
/// The boolean value true.
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct True {
    
}

impl True {
    pub fn new() -> Self {
        Self {
            
        }
    }
}
