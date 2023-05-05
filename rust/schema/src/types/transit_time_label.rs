use crate::prelude::*;

use super::delivery_time_settings::DeliveryTimeSettings;
use super::offer_shipping_details::OfferShippingDetails;

/// Label to match an <a class="localLink" href="/OfferShippingDetails">OfferShippingDetails</a> with a <a class="localLink" href="/DeliveryTimeSettings">DeliveryTimeSettings</a> (within the context of a <a class="localLink" href="/shippingSettingsLink">shippingSettingsLink</a> cross-reference).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum transitTimeLabel {
    DeliveryTimeSettings(DeliveryTimeSettings),
    OfferShippingDetails(OfferShippingDetails),
}
