use crate::prelude::*;

use super::demand::Demand;
use super::offer::Offer;
use super::product::Product;

/// The Stock Keeping Unit (SKU), i.e. a merchant-specific identifier for a product or service, or the product to which the offer refers.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum sku {
    Demand(Demand),
    Offer(Offer),
    Product(Product),
}
