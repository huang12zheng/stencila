use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// The group the release is credited to if different than the byArtist. For example, Red and Blue is credited to "Stefani Germanotta Band", but by Lady Gaga.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum creditedTo {
    Organization(Organization),
    Person(Person),
}
