use crate::prelude::*;

use super::dietary_supplement::DietarySupplement;
use super::drug::Drug;

/// The specific biochemical interaction through which this drug or supplement produces its pharmacological effect.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum mechanismOfAction {
    DietarySupplement(DietarySupplement),
    Drug(Drug),
}
