// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;



/// https://schema.org/XPathType
/// * MOD OF:
/// https://pending.schema.org
/// * COMMENT:
/// Text representing an XPath (typically but not necessarily version 1.0).
/// * EXTEND FROM:
/// https://schema.org/Text
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct XPathType {
    
}

impl XPathType {
    pub fn new() -> Self {
        Self {
            
        }
    }
}
