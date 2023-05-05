use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/composer
/// The person or organization who wrote a composition, or who is the composer of a work performed at some event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ComposerPropEnum {
    Organization(Organization),
    Person(Person),
}
