use crate::prelude::*;

use super::product::Product;
use super::vehicle::Vehicle;

/// The date the item, e.g. vehicle, was purchased by the current owner.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum purchaseDate {
    Product(Product),
    Vehicle(Vehicle),
}
