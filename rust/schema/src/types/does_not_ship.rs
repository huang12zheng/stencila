use crate::prelude::*;

use super::offer_shipping_details::OfferShippingDetails;
use super::shipping_rate_settings::ShippingRateSettings;

/// Indicates when shipping to a particular <a class="localLink" href="/shippingDestination">shippingDestination</a> is not available.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum doesNotShip {
    OfferShippingDetails(OfferShippingDetails),
    ShippingRateSettings(ShippingRateSettings),
}
