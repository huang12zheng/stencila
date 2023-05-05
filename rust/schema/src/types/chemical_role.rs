use crate::prelude::*;

use super::chemical_substance::ChemicalSubstance;
use super::molecular_entity::MolecularEntity;

/// A role played by the BioChemEntity within a chemical context.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum chemicalRole {
    ChemicalSubstance(ChemicalSubstance),
    MolecularEntity(MolecularEntity),
}
