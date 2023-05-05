use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// The (e.g. fictional) character, Person or Organization to whom the quotation is attributed within the containing CreativeWork.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum spokenByCharacter {
    Organization(Organization),
    Person(Person),
}
