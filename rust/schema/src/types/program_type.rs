use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;

/// The type of educational or occupational program. For example, classroom, internship, alternance, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum programType {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
