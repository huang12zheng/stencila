use crate::prelude::*;

use super::drug::Drug;
use super::drug_cost::DrugCost;

/// The unit in which the drug is measured, e.g. '5 mg tablet'.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum drugUnit {
    Drug(Drug),
    DrugCost(DrugCost),
}
