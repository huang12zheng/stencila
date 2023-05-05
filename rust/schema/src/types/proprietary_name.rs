use crate::prelude::*;

use super::dietary_supplement::DietarySupplement;
use super::drug::Drug;

/// Proprietary name given to the diet plan, typically by its originator or creator.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum proprietaryName {
    DietarySupplement(DietarySupplement),
    Drug(Drug),
}
