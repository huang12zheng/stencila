use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// The publisher of the creative work.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum publisher {
    Organization(Organization),
    Person(Person),
}
