use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;

/// The job title of the person (for example, Financial Manager).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum jobTitle {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
