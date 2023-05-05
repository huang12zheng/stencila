use crate::prelude::*;

use super::demand::Demand;
use super::offer::Offer;

/// The warranty promise(s) included in the offer.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum warranty {
    Demand(Demand),
    Offer(Offer),
}
