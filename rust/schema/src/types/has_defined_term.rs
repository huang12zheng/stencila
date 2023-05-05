use crate::prelude::*;

use super::defined_term_set::DefinedTermSet;
use super::taxon::Taxon;

/// A Defined Term contained in this term set.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum hasDefinedTerm {
    DefinedTermSet(DefinedTermSet),
    Taxon(Taxon),
}
