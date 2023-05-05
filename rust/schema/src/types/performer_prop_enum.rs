use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/performer
/// A performer at the event&#x2014;for example, a presenter, musician, musical group or actor.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum PerformerPropEnum {
    Organization(Organization),
    Person(Person),
}
