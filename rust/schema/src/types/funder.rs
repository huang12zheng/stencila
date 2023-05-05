use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// A person or organization that supports (sponsors) something through some kind of financial contribution.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum funder {
    Organization(Organization),
    Person(Person),
}
