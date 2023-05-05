use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// A sub property of participant. The participant/person/organization that bought the object.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum buyer {
    Organization(Organization),
    Person(Person),
}
