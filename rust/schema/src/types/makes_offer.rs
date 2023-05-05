use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// A pointer to products or services offered by the organization or person.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum makesOffer {
    Organization(Organization),
    Person(Person),
}
