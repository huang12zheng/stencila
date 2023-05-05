use crate::prelude::*;

use super::demand::Demand;
use super::offer::Offer;
use super::some_products::SomeProducts;

/// The current approximate inventory level for the item or items.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum inventoryLevel {
    Demand(Demand),
    Offer(Offer),
    SomeProducts(SomeProducts),
}
