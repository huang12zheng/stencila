use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// The creator/author of this CreativeWork. This is the same as the Author property for CreativeWork.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum creator {
    Organization(Organization),
    Person(Person),
}
