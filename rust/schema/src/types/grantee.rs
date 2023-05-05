use crate::prelude::*;

use super::audience::Audience;
use super::contact_point::ContactPoint;
use super::organization::Organization;
use super::person::Person;

/// The person, organization, contact point, or audience that has been granted this permission.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum grantee {
    Audience(Audience),
    ContactPoint(ContactPoint),
    Organization(Organization),
    Person(Person),
}
