use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/funder
/// A person or organization that supports (sponsors) something through some kind of financial contribution.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum FunderPropEnum {
    Organization(Organization),
    Person(Person),
}
