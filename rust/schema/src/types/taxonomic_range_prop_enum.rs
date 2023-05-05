use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::taxon::Taxon;
use super::text::Text;
use super::url::URL;


/// http://schema.org/taxonomicRange
/// The taxonomic grouping of the organism that expresses, encodes, or in some way related to the BioChemEntity.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum TaxonomicRangePropEnum {
    DefinedTerm(DefinedTerm),
    Taxon(Taxon),
    Text(Text),
    URL(URL),
}
