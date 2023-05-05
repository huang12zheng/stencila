use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/endorsers
/// People or organizations that endorse the plan.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum EndorsersPropEnum {
    Organization(Organization),
    Person(Person),
}
