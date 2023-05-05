use crate::prelude::*;

use super::audience::Audience;
use super::organization::Organization;
use super::person::Person;


/// http://schema.org/sender
/// A sub property of participant. The participant who is at the sending end of the action.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SenderPropEnum {
    Audience(Audience),
    Organization(Organization),
    Person(Person),
}
