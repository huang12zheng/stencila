use crate::prelude::*;

use super::defined_term_set::DefinedTermSet;
use super::url::URL;

/// A <a class="localLink" href="/DefinedTermSet">DefinedTermSet</a> that contains this term.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum inDefinedTermSet {
    DefinedTermSet(DefinedTermSet),
    URL(URL),
}
