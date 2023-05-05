use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// A performer at the event&#x2014;for example, a presenter, musician, musical group or actor.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum performer {
    Organization(Organization),
    Person(Person),
}
