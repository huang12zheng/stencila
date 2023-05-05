use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// The person or organization the reservation or ticket is for.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum underName {
    Organization(Organization),
    Person(Person),
}
