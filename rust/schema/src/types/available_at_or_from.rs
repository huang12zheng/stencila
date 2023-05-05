use crate::prelude::*;

use super::demand::Demand;
use super::offer::Offer;

/// The place(s) from which the offer can be obtained (e.g. store locations).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum availableAtOrFrom {
    Demand(Demand),
    Offer(Offer),
}
