use crate::prelude::*;

use super::health_plan_formulary::HealthPlanFormulary;
use super::health_plan_network::HealthPlanNetwork;

/// The costs to the patient for services under this network or formulary.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum healthPlanCostSharing {
    HealthPlanFormulary(HealthPlanFormulary),
    HealthPlanNetwork(HealthPlanNetwork),
}
