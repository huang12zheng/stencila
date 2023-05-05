use crate::prelude::*;

use super::demand::Demand;
use super::offer::Offer;

/// The payment method(s) accepted by seller for this offer.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum acceptedPaymentMethod {
    Demand(Demand),
    Offer(Offer),
}
