use crate::prelude::*;

use super::product::Product;
use super::service::Service;

/// A pointer to another, somehow related product (or multiple products).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum isRelatedTo {
    Product(Product),
    Service(Service),
}
