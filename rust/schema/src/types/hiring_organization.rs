use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// Organization or Person offering the job position.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum hiringOrganization {
    Organization(Organization),
    Person(Person),
}
