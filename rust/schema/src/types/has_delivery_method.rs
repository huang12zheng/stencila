use crate::prelude::*;

use super::delivery_event::DeliveryEvent;
use super::parcel_delivery::ParcelDelivery;

/// Method used for delivery or shipping.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum hasDeliveryMethod {
    DeliveryEvent(DeliveryEvent),
    ParcelDelivery(ParcelDelivery),
}
