use crate::prelude::*;

use super::educational_organization::EducationalOrganization;
use super::organization::Organization;


/// http://schema.org/alumniOf
/// An organization that the person is an alumni of.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AlumniOfPropEnum {
    EducationalOrganization(EducationalOrganization),
    Organization(Organization),
}
