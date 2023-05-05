use crate::prelude::*;

use super::dietary_supplement::DietarySupplement;
use super::drug::Drug;
use super::drug_strength::DrugStrength;
use super::substance::Substance;

/// An active ingredient, typically chemical compounds and/or biologic substances.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum activeIngredient {
    DietarySupplement(DietarySupplement),
    Drug(Drug),
    DrugStrength(DrugStrength),
    Substance(Substance),
}
