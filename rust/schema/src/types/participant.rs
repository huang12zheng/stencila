use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// Other co-agents that participated in the action indirectly. E.g. John wrote a book with <em>Steve</em>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum participant {
    Organization(Organization),
    Person(Person),
}
