use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/underName
/// The person or organization the reservation or ticket is for.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum UnderNamePropEnum {
    Organization(Organization),
    Person(Person),
}
