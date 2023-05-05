use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/lender
/// A sub property of participant. The person that lends the object being borrowed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LenderPropEnum {
    Organization(Organization),
    Person(Person),
}
