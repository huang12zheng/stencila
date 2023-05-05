// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;



/// * COMMENT: A point in time recurring on multiple days in the form hh:mm:ss[Z|(+|-)hh:mm] (see <a href="http://www.w3.org/TR/xmlschema-2/#time">XML schema for details</a>).
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Time {
    
}

impl Time {
    pub fn new() -> Self {
        Self {
            
        }
    }
}
