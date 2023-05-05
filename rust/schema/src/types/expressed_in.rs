use crate::prelude::*;

use super::anatomical_structure::AnatomicalStructure;
use super::anatomical_system::AnatomicalSystem;
use super::bio_chem_entity::BioChemEntity;
use super::defined_term::DefinedTerm;

/// Tissue, organ, biological sample, etc in which activity of this gene has been observed experimentally. For example brain, digestive system.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum expressedIn {
    AnatomicalStructure(AnatomicalStructure),
    AnatomicalSystem(AnatomicalSystem),
    BioChemEntity(BioChemEntity),
    DefinedTerm(DefinedTerm),
}
