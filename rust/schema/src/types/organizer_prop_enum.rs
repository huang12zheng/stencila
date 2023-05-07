use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/organizer
/// An organizer of an Event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum OrganizerPropEnum {
    Organization(Organization),
    Person(Person),
}