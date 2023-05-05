// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;



/// https://schema.org/Text
/// * COMMENT:
/// Data type: Text.
/// * LOOK ALSO:
/// https://schema.org/CssSelectorType, https://schema.org/PronounceableText, https://schema.org/URL, https://schema.org/XPathType
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Text {
    
}

impl Text {
    pub fn new() -> Self {
        Self {
            
        }
    }
}
