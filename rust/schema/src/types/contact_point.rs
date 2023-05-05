use crate::prelude::*;

use super::health_insurance_plan::HealthInsurancePlan;
use super::organization::Organization;
use super::person::Person;

/// A contact point for a person or organization.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum contactPoint {
    HealthInsurancePlan(HealthInsurancePlan),
    Organization(Organization),
    Person(Person),
}
