use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;

/// The status of a creative work in terms of its stage in a lifecycle. Example terms include Incomplete, Draft, Published, Obsolete. Some organizations define a set of terms for the stages of their publication lifecycle.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum creativeWorkStatus {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
