use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// The Value-added Tax ID of the organization or person.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum vatID {
    Organization(Organization),
    Person(Person),
}
