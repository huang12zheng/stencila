use crate::prelude::*;

use super::audience::Audience;
use super::organization::Organization;
use super::person::Person;

/// A sub property of participant. The participant who is at the sending end of the action.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum sender {
    Audience(Audience),
    Organization(Organization),
    Person(Person),
}
