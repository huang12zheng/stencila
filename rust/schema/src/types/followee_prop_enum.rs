use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/followee
/// A sub property of object. The person or organization being followed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum FolloweePropEnum {
    Organization(Organization),
    Person(Person),
}
