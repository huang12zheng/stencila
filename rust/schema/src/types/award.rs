use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::organization::Organization;
use super::person::Person;
use super::product::Product;
use super::service::Service;

/// An award won by or for this item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum award {
    CreativeWork(CreativeWork),
    Organization(Organization),
    Person(Person),
    Product(Product),
    Service(Service),
}
