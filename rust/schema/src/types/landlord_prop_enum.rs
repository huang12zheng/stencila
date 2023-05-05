use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/landlord
/// A sub property of participant. The owner of the real estate property.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LandlordPropEnum {
    Organization(Organization),
    Person(Person),
}
