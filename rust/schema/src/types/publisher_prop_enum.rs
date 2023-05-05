use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/publisher
/// The publisher of the creative work.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum PublisherPropEnum {
    Organization(Organization),
    Person(Person),
}
