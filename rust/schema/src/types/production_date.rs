use crate::prelude::*;

use super::product::Product;
use super::vehicle::Vehicle;

/// The date of production of the item, e.g. vehicle.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum productionDate {
    Product(Product),
    Vehicle(Vehicle),
}
