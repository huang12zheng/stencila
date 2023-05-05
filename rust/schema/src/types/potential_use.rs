use crate::prelude::*;

use super::chemical_substance::ChemicalSubstance;
use super::molecular_entity::MolecularEntity;

/// Intended use of the BioChemEntity by humans.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum potentialUse {
    ChemicalSubstance(ChemicalSubstance),
    MolecularEntity(MolecularEntity),
}
