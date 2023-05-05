use crate::prelude::*;

use super::brand::Brand;
use super::organization::Organization;
use super::place::Place;
use super::product::Product;
use super::service::Service;

/// A slogan or motto associated with the item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum slogan {
    Brand(Brand),
    Organization(Organization),
    Place(Place),
    Product(Product),
    Service(Service),
}
