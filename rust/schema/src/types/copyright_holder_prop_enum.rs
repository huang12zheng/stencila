use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/copyrightHolder
/// The party holding the legal copyright to the CreativeWork.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CopyrightHolderPropEnum {
    Organization(Organization),
    Person(Person),
}
