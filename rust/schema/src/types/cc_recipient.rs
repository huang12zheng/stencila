use crate::prelude::*;

use super::contact_point::ContactPoint;
use super::organization::Organization;
use super::person::Person;

/// A sub property of recipient. The recipient copied on a message.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum ccRecipient {
    ContactPoint(ContactPoint),
    Organization(Organization),
    Person(Person),
}
