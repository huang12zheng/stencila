use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/creator
/// The creator/author of this CreativeWork. This is the same as the Author property for CreativeWork.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CreatorPropEnum {
    Organization(Organization),
    Person(Person),
}
