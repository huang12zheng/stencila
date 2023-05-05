use crate::prelude::*;

use super::contact_point::ContactPoint;
use super::organization::Organization;
use super::person::Person;


/// http://schema.org/ccRecipient
/// A sub property of recipient. The recipient copied on a message.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CcRecipientPropEnum {
    ContactPoint(ContactPoint),
    Organization(Organization),
    Person(Person),
}
