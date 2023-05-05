use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// An Organization (or ProgramMembership) to which this Person or Organization belongs.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum memberOf {
    Organization(Organization),
    Person(Person),
}
