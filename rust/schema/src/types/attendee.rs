use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// A person or organization attending the event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum attendee {
    Organization(Organization),
    Person(Person),
}
