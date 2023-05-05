// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;



/// https://schema.org/Number
/// * COMMENT:
/// Data type: Number.<br/><br/>
/// 
/// Usage guidelines:<br/><br/>
/// 
/// <ul>
/// <li>Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similar Unicode symbols.</li>
/// <li>Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.</li>
/// </ul>
/// * LOOK ALSO:
/// https://schema.org/Float, https://schema.org/Integer
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Number {
    
}

impl Number {
    pub fn new() -> Self {
        Self {
            
        }
    }
}
