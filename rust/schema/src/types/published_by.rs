use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// An agent associated with the publication event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum publishedBy {
    Organization(Organization),
    Person(Person),
}
