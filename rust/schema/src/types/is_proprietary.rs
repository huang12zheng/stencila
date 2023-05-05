use crate::prelude::*;

use super::dietary_supplement::DietarySupplement;
use super::drug::Drug;

/// True if this item's name is a proprietary/brand name (vs. generic name).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum isProprietary {
    DietarySupplement(DietarySupplement),
    Drug(Drug),
}
