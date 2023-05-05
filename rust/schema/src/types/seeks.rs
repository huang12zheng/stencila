use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// A pointer to products or services sought by the organization or person (demand).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum seeks {
    Organization(Organization),
    Person(Person),
}
