use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;

/// The purpose of a work in the context of education; for example, 'assignment', 'group work'.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum educationalUse {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
