use crate::prelude::*;

use super::demand::Demand;
use super::offer::Offer;

/// The availability of this item&#x2014;for example In stock, Out of stock, Pre-order, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum availability {
    Demand(Demand),
    Offer(Offer),
}
