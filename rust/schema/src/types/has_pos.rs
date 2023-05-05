use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// Points-of-Sales operated by the organization or person.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum hasPOS {
    Organization(Organization),
    Person(Person),
}
