use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/agent
/// The direct performer or driver of the action (animate or inanimate). E.g. <em>John</em> wrote a book.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AgentPropEnum {
    Organization(Organization),
    Person(Person),
}
