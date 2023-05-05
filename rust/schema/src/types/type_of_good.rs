use crate::prelude::*;

use super::product::Product;
use super::service::Service;

/// The product that this structured value is referring to.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum typeOfGood {
    Product(Product),
    Service(Service),
}
