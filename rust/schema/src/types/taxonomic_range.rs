use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::taxon::Taxon;
use super::text::Text;
use super::url::URL;

/// The taxonomic grouping of the organism that expresses, encodes, or in some way related to the BioChemEntity.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum taxonomicRange {
    DefinedTerm(DefinedTerm),
    Taxon(Taxon),
    Text(Text),
    URL(URL),
}
