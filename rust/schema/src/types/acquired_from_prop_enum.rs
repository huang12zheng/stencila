use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/acquiredFrom
/// The organization or person from which the product was acquired.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AcquiredFromPropEnum {
    Organization(Organization),
    Person(Person),
}
