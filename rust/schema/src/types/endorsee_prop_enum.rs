use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/endorsee
/// A sub property of participant. The person/organization being supported.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum EndorseePropEnum {
    Organization(Organization),
    Person(Person),
}
