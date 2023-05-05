use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/sponsor
/// A person or organization that supports a thing through a pledge, promise, or financial contribution. E.g. a sponsor of a Medical Study or a corporate sponsor of an event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SponsorPropEnum {
    Organization(Organization),
    Person(Person),
}
