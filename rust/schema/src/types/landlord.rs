use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// A sub property of participant. The owner of the real estate property.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum landlord {
    Organization(Organization),
    Person(Person),
}
