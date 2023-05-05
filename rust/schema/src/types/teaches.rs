use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;

/// The item being described is intended to help a person learn the competency or learning outcome defined by the referenced term.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum teaches {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
