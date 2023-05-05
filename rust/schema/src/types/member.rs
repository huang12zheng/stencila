use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// A member of an Organization or a ProgramMembership. Organizations can be members of organizations; ProgramMembership is typically for individuals.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum member {
    Organization(Organization),
    Person(Person),
}
