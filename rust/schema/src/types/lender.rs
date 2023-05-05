use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// A sub property of participant. The person that lends the object being borrowed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum lender {
    Organization(Organization),
    Person(Person),
}
