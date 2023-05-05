use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// The Dun &amp; Bradstreet DUNS number for identifying an organization or business person.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum duns {
    Organization(Organization),
    Person(Person),
}
