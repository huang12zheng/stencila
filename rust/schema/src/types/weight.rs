use crate::prelude::*;

use super::offer_shipping_details::OfferShippingDetails;
use super::person::Person;
use super::product::Product;

/// The weight of the product or person.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum weight {
    OfferShippingDetails(OfferShippingDetails),
    Person(Person),
    Product(Product),
}
