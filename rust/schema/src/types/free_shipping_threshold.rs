use crate::prelude::*;

use super::delivery_charge_specification::DeliveryChargeSpecification;
use super::monetary_amount::MonetaryAmount;

/// A monetary value above (or at) which the shipping rate becomes free. Intended to be used via an <a class="localLink" href="/OfferShippingDetails">OfferShippingDetails</a> with <a class="localLink" href="/shippingSettingsLink">shippingSettingsLink</a> matching this <a class="localLink" href="/ShippingRateSettings">ShippingRateSettings</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum freeShippingThreshold {
    DeliveryChargeSpecification(DeliveryChargeSpecification),
    MonetaryAmount(MonetaryAmount),
}
