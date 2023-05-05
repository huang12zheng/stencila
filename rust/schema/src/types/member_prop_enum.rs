use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/member
/// A member of an Organization or a ProgramMembership. Organizations can be members of organizations; ProgramMembership is typically for individuals.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum MemberPropEnum {
    Organization(Organization),
    Person(Person),
}
