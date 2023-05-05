use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// People or organizations that endorse the plan.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum endorsers {
    Organization(Organization),
    Person(Person),
}
