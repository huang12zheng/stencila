use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// People or organizations that have reviewed the content on this web page for accuracy and/or completeness.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum reviewedBy {
    Organization(Organization),
    Person(Person),
}
