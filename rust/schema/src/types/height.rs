use crate::prelude::*;

use super::media_object::MediaObject;
use super::offer_shipping_details::OfferShippingDetails;
use super::person::Person;
use super::product::Product;
use super::visual_artwork::VisualArtwork;

/// The height of the item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum height {
    MediaObject(MediaObject),
    OfferShippingDetails(OfferShippingDetails),
    Person(Person),
    Product(Product),
    VisualArtwork(VisualArtwork),
}
