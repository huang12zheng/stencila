use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;
use super::service::Service;

/// Indicates an OfferCatalog listing for this Organization, Person, or Service.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum hasOfferCatalog {
    Organization(Organization),
    Person(Person),
    Service(Service),
}
