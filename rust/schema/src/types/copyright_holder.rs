use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// The party holding the legal copyright to the CreativeWork.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum copyrightHolder {
    Organization(Organization),
    Person(Person),
}
