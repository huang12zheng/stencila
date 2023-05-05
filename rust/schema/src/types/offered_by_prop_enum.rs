use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/offeredBy
/// A pointer to the organization or person making the offer.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum OfferedByPropEnum {
    Organization(Organization),
    Person(Person),
}
