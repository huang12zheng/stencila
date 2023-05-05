use crate::prelude::*;

use super::offer::Offer;
use super::product::Product;

/// Used to tag an item to be intended or suitable for consumption or use by adults only.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum hasAdultConsideration {
    Offer(Offer),
    Product(Product),
}
