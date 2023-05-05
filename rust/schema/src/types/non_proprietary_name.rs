use crate::prelude::*;

use super::dietary_supplement::DietarySupplement;
use super::drug::Drug;

/// The generic name of this drug or supplement.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum nonProprietaryName {
    DietarySupplement(DietarySupplement),
    Drug(Drug),
}
