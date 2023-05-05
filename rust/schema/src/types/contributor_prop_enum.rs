use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/contributor
/// A secondary contributor to the CreativeWork or Event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ContributorPropEnum {
    Organization(Organization),
    Person(Person),
}
