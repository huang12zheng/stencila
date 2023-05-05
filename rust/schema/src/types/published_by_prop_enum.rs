use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/publishedBy
/// An agent associated with the publication event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum PublishedByPropEnum {
    Organization(Organization),
    Person(Person),
}
