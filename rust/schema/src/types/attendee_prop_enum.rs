use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/attendee
/// A person or organization attending the event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AttendeePropEnum {
    Organization(Organization),
    Person(Person),
}
