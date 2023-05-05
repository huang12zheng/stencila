use crate::prelude::*;

use super::demand::Demand;
use super::offer::Offer;
use super::product_collection::ProductCollection;

/// This links to a node or nodes indicating the exact quantity of the products included in  an <a class="localLink" href="/Offer">Offer</a> or <a class="localLink" href="/ProductCollection">ProductCollection</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum includesObject {
    Demand(Demand),
    Offer(Offer),
    ProductCollection(ProductCollection),
}
