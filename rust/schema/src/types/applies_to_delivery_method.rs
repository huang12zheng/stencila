use crate::prelude::*;

use super::delivery_charge_specification::DeliveryChargeSpecification;
use super::payment_charge_specification::PaymentChargeSpecification;

/// The delivery method(s) to which the delivery charge or payment charge specification applies.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum appliesToDeliveryMethod {
    DeliveryChargeSpecification(DeliveryChargeSpecification),
    PaymentChargeSpecification(PaymentChargeSpecification),
}
