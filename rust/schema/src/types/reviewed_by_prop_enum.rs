use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/reviewedBy
/// People or organizations that have reviewed the content on this web page for accuracy and/or completeness.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ReviewedByPropEnum {
    Organization(Organization),
    Person(Person),
}
