use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// The North American Industry Classification System (NAICS) code for a particular organization or business person.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum naics {
    Organization(Organization),
    Person(Person),
}
