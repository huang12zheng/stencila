use crate::prelude::*;

use super::delivery_time_settings::DeliveryTimeSettings;
use super::shipping_rate_settings::ShippingRateSettings;

/// This can be marked 'true' to indicate that some published <a class="localLink" href="/DeliveryTimeSettings">DeliveryTimeSettings</a> or <a class="localLink" href="/ShippingRateSettings">ShippingRateSettings</a> are intended to apply to all <a class="localLink" href="/OfferShippingDetails">OfferShippingDetails</a> published by the same merchant, when referenced by a <a class="localLink" href="/shippingSettingsLink">shippingSettingsLink</a> in those settings. It is not meaningful to use a 'true' value for this property alongside a transitTimeLabel (for <a class="localLink" href="/DeliveryTimeSettings">DeliveryTimeSettings</a>) or shippingLabel (for <a class="localLink" href="/ShippingRateSettings">ShippingRateSettings</a>), since this property is for use with unlabelled settings.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum isUnlabelledFallback {
    DeliveryTimeSettings(DeliveryTimeSettings),
    ShippingRateSettings(ShippingRateSettings),
}
