use crate::prelude::*;

use super::educational_organization::EducationalOrganization;
use super::organization::Organization;

/// Alumni of an organization.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum alumni {
    EducationalOrganization(EducationalOrganization),
    Organization(Organization),
}
