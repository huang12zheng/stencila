use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// The person or organization who wrote a composition, or who is the composer of a work performed at some event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum composer {
    Organization(Organization),
    Person(Person),
}
