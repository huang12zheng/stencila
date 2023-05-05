use crate::prelude::*;

use super::demand::Demand;
use super::offer::Offer;

/// The delivery method(s) available for this offer.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum availableDeliveryMethod {
    Demand(Demand),
    Offer(Offer),
}
