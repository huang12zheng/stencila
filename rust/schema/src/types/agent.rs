use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// The direct performer or driver of the action (animate or inanimate). E.g. <em>John</em> wrote a book.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum agent {
    Organization(Organization),
    Person(Person),
}
