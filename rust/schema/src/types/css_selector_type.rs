// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;



/// * MOD OF: https://pending.schema.org * COMMENT: Text representing a CSS selector. * EXTEND FROM: https://schema.org/Text
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct CssSelectorType {
    
}

impl CssSelectorType {
    pub fn new() -> Self {
        Self {
            
        }
    }
}
