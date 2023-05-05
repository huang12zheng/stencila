use crate::prelude::*;

use super::dietary_supplement::DietarySupplement;
use super::dose_schedule::DoseSchedule;

/// Characteristics of the population for which this is intended, or which typically uses it, e.g. 'adults'.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum targetPopulation {
    DietarySupplement(DietarySupplement),
    DoseSchedule(DoseSchedule),
}
