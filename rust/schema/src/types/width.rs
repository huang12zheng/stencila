use crate::prelude::*;

use super::media_object::MediaObject;
use super::offer_shipping_details::OfferShippingDetails;
use super::product::Product;
use super::visual_artwork::VisualArtwork;

/// The width of the item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum width {
    MediaObject(MediaObject),
    OfferShippingDetails(OfferShippingDetails),
    Product(Product),
    VisualArtwork(VisualArtwork),
}
