use crate::prelude::*;

use super::audience::Audience;
use super::contact_point::ContactPoint;
use super::organization::Organization;
use super::person::Person;

/// A sub property of recipient. The recipient who was directly sent the message.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum toRecipient {
    Audience(Audience),
    ContactPoint(ContactPoint),
    Organization(Organization),
    Person(Person),
}
