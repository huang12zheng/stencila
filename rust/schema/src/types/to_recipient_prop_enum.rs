use crate::prelude::*;

use super::audience::Audience;
use super::contact_point::ContactPoint;
use super::organization::Organization;
use super::person::Person;


/// http://schema.org/toRecipient
/// A sub property of recipient. The recipient who was directly sent the message.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ToRecipientPropEnum {
    Audience(Audience),
    ContactPoint(ContactPoint),
    Organization(Organization),
    Person(Person),
}
