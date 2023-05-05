use crate::prelude::*;

use super::contact_point::ContactPoint;
use super::organization::Organization;
use super::person::Person;

/// Email address.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum email {
    ContactPoint(ContactPoint),
    Organization(Organization),
    Person(Person),
}
