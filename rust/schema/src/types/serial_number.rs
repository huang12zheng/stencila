use crate::prelude::*;

use super::demand::Demand;
use super::individual_product::IndividualProduct;
use super::offer::Offer;

/// The serial number or any alphanumeric identifier of a particular product. When attached to an offer, it is a shortcut for the serial number of the product included in the offer.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum serialNumber {
    Demand(Demand),
    IndividualProduct(IndividualProduct),
    Offer(Offer),
}
