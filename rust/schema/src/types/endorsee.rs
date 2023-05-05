use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// A sub property of participant. The person/organization being supported.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum endorsee {
    Organization(Organization),
    Person(Person),
}
