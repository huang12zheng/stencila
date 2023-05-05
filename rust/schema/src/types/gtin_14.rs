use crate::prelude::*;

use super::demand::Demand;
use super::offer::Offer;
use super::product::Product;

/// The GTIN-14 code of the product, or the product to which the offer refers. See <a href="http://www.gs1.org/barcodes/technical/idkeys/gtin">GS1 GTIN Summary</a> for more details.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum gtin14 {
    Demand(Demand),
    Offer(Offer),
    Product(Product),
}
