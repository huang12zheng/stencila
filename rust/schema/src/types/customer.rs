use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// Party placing the order or paying the invoice.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum customer {
    Organization(Organization),
    Person(Person),
}
