use crate::prelude::*;

use super::dietary_supplement::DietarySupplement;
use super::drug::Drug;
use super::drug_strength::DrugStrength;
use super::substance::Substance;

/// Recommended intake of this supplement for a given population as defined by a specific recommending authority.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum maximumIntake {
    DietarySupplement(DietarySupplement),
    Drug(Drug),
    DrugStrength(DrugStrength),
    Substance(Substance),
}
