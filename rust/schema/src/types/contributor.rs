use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// A secondary contributor to the CreativeWork or Event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum contributor {
    Organization(Organization),
    Person(Person),
}
