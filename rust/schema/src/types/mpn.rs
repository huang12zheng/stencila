use crate::prelude::*;

use super::demand::Demand;
use super::offer::Offer;
use super::product::Product;

/// The Manufacturer Part Number (MPN) of the product, or the product to which the offer refers.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum mpn {
    Demand(Demand),
    Offer(Offer),
    Product(Product),
}
