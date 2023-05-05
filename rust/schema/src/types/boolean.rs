// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;



/// https://schema.org/Boolean
/// * COMMENT:
/// Boolean: True or False.
/// * LOOK ALSO:
/// https://schema.org/False, https://schema.org/True
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Boolean {
    
}

impl Boolean {
    pub fn new() -> Self {
        Self {
            
        }
    }
}
