use crate::prelude::*;

use super::delivery_time_settings::DeliveryTimeSettings;
use super::offer_shipping_details::OfferShippingDetails;

/// The total delay between the receipt of the order and the goods reaching the final customer.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum deliveryTime {
    DeliveryTimeSettings(DeliveryTimeSettings),
    OfferShippingDetails(OfferShippingDetails),
}
