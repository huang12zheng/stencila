use crate::prelude::*;

use super::drug::Drug;
use super::therapeutic_procedure::TherapeuticProcedure;

/// A dosing schedule for the drug for a given population, either observed, recommended, or maximum dose based on the type used.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum doseSchedule {
    Drug(Drug),
    TherapeuticProcedure(TherapeuticProcedure),
}
