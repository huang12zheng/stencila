use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/translator
/// Organization or person who adapts a creative work to different languages, regional differences and technical requirements of a target market, or that translates during some event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum TranslatorPropEnum {
    Organization(Organization),
    Person(Person),
}
