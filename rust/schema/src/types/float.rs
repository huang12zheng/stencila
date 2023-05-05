// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;



/// https://schema.org/Float
/// * COMMENT:
/// Data type: Floating number.
/// * EXTEND FROM:
/// https://schema.org/Number
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Float {
    
}

impl Float {
    pub fn new() -> Self {
        Self {
            
        }
    }
}
