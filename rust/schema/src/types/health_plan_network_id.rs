use crate::prelude::*;

use super::health_plan_network::HealthPlanNetwork;
use super::medical_organization::MedicalOrganization;

/// Name or unique ID of network. (Networks are often reused across different insurance plans.)
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum healthPlanNetworkId {
    HealthPlanNetwork(HealthPlanNetwork),
    MedicalOrganization(MedicalOrganization),
}
