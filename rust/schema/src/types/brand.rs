use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;
use super::product::Product;
use super::service::Service;

/// The brand(s) associated with a product or service, or the brand(s) maintained by an organization or business person.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum brand {
    Organization(Organization),
    Person(Person),
    Product(Product),
    Service(Service),
}
