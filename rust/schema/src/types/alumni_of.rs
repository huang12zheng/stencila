use crate::prelude::*;

use super::educational_organization::EducationalOrganization;
use super::organization::Organization;

/// An organization that the person is an alumni of.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum alumniOf {
    EducationalOrganization(EducationalOrganization),
    Organization(Organization),
}
