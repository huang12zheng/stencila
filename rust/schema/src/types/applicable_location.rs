use crate::prelude::*;

use super::drug_cost::DrugCost;
use super::drug_legal_status::DrugLegalStatus;

/// The location in which the status applies.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum applicableLocation {
    DrugCost(DrugCost),
    DrugLegalStatus(DrugLegalStatus),
}
