use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// A pointer to the organization or person making the offer.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum offeredBy {
    Organization(Organization),
    Person(Person),
}
