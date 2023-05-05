use crate::prelude::*;

use super::delivery_time_settings::DeliveryTimeSettings;
use super::offer_shipping_details::OfferShippingDetails;
use super::shipping_rate_settings::ShippingRateSettings;

/// indicates (possibly multiple) shipping destinations. These can be defined in several ways, e.g. postalCode ranges.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum shippingDestination {
    DeliveryTimeSettings(DeliveryTimeSettings),
    OfferShippingDetails(OfferShippingDetails),
    ShippingRateSettings(ShippingRateSettings),
}
