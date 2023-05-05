use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// A sub property of object. The person or organization being followed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum followee {
    Organization(Organization),
    Person(Person),
}
