use crate::prelude::*;

use super::offer_shipping_details::OfferShippingDetails;
use super::product::Product;
use super::visual_artwork::VisualArtwork;

/// The depth of the item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum depth {
    OfferShippingDetails(OfferShippingDetails),
    Product(Product),
    VisualArtwork(VisualArtwork),
}
