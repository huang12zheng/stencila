use crate::prelude::*;

use super::offer_shipping_details::OfferShippingDetails;
use super::shipping_rate_settings::ShippingRateSettings;

/// Label to match an <a class="localLink" href="/OfferShippingDetails">OfferShippingDetails</a> with a <a class="localLink" href="/ShippingRateSettings">ShippingRateSettings</a> (within the context of a <a class="localLink" href="/shippingSettingsLink">shippingSettingsLink</a> cross-reference).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum shippingLabel {
    OfferShippingDetails(OfferShippingDetails),
    ShippingRateSettings(ShippingRateSettings),
}
