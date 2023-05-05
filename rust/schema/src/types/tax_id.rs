use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// The Tax / Fiscal ID of the organization or person, e.g. the TIN in the US or the CIF/NIF in Spain.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum taxID {
    Organization(Organization),
    Person(Person),
}
