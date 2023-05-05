use crate::prelude::*;

use super::contact_point::ContactPoint;
use super::organization::Organization;
use super::person::Person;
use super::place::Place;

/// The fax number.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum faxNumber {
    ContactPoint(ContactPoint),
    Organization(Organization),
    Person(Person),
    Place(Place),
}
