use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/hiringOrganization
/// Organization or Person offering the job position.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum HiringOrganizationPropEnum {
    Organization(Organization),
    Person(Person),
}
