use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;

/// The industry associated with the job position.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum industry {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
