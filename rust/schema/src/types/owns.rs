use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// Products owned by the organization or person.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum owns {
    Organization(Organization),
    Person(Person),
}
