use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// The organization or person from which the product was acquired.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum acquiredFrom {
    Organization(Organization),
    Person(Person),
}
