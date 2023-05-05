// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;



/// * COMMENT: The boolean value false.
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct False {
    
}

impl False {
    pub fn new() -> Self {
        Self {
            
        }
    }
}
