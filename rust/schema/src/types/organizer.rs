use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// An organizer of an Event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum organizer {
    Organization(Organization),
    Person(Person),
}
