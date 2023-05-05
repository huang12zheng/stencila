use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/buyer
/// A sub property of participant. The participant/person/organization that bought the object.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum BuyerPropEnum {
    Organization(Organization),
    Person(Person),
}
