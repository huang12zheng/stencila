use crate::prelude::*;

use super::audience::Audience;
use super::contact_point::ContactPoint;
use super::organization::Organization;
use super::person::Person;


/// http://schema.org/grantee
/// The person, organization, contact point, or audience that has been granted this permission.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum GranteePropEnum {
    Audience(Audience),
    ContactPoint(ContactPoint),
    Organization(Organization),
    Person(Person),
}
