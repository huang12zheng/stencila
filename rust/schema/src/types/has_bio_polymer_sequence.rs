use crate::prelude::*;

use super::gene::Gene;
use super::protein::Protein;

/// A symbolic representation of a BioChemEntity. For example, a nucleotide sequence of a Gene or an amino acid sequence of a Protein.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum hasBioPolymerSequence {
    Gene(Gene),
    Protein(Protein),
}
