use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/creditedTo
/// The group the release is credited to if different than the byArtist. For example, Red and Blue is credited to "Stefani Germanotta Band", but by Lady Gaga.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CreditedToPropEnum {
    Organization(Organization),
    Person(Person),
}
