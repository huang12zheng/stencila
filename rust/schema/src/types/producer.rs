use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// The person or organization who produced the work (e.g. music album, movie, TV/radio series etc.).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum producer {
    Organization(Organization),
    Person(Person),
}
