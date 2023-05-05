use crate::prelude::*;

use super::health_insurance_plan::HealthInsurancePlan;
use super::health_plan_formulary::HealthPlanFormulary;

/// The tier(s) of drugs offered by this formulary or insurance plan.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum healthPlanDrugTier {
    HealthInsurancePlan(HealthInsurancePlan),
    HealthPlanFormulary(HealthPlanFormulary),
}
