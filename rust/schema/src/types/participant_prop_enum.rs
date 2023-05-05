use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/participant
/// Other co-agents that participated in the action indirectly. E.g. John wrote a book with <em>Steve</em>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ParticipantPropEnum {
    Organization(Organization),
    Person(Person),
}
