use crate::prelude::*;

use super::offer_shipping_details::OfferShippingDetails;
use super::shipping_rate_settings::ShippingRateSettings;

/// The shipping rate is the cost of shipping to the specified destination. Typically, the maxValue and currency values (of the <a class="localLink" href="/MonetaryAmount">MonetaryAmount</a>) are most appropriate.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum shippingRate {
    OfferShippingDetails(OfferShippingDetails),
    ShippingRateSettings(ShippingRateSettings),
}
