use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/spokenByCharacter
/// The (e.g. fictional) character, Person or Organization to whom the quotation is attributed within the containing CreativeWork.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SpokenByCharacterPropEnum {
    Organization(Organization),
    Person(Person),
}
