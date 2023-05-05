use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;

/// A statement of knowledge, skill, ability, task or any other assertion expressing a competency that is desired or required to fulfill this role or to work in this occupation.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum skills {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
