use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/producer
/// The person or organization who produced the work (e.g. music album, movie, TV/radio series etc.).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ProducerPropEnum {
    Organization(Organization),
    Person(Person),
}
