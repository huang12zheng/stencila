use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// A person or organization that supports a thing through a pledge, promise, or financial contribution. E.g. a sponsor of a Medical Study or a corporate sponsor of an event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum sponsor {
    Organization(Organization),
    Person(Person),
}
