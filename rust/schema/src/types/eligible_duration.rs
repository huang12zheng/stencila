use crate::prelude::*;

use super::demand::Demand;
use super::offer::Offer;

/// The duration for which the given offer is valid.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum eligibleDuration {
    Demand(Demand),
    Offer(Offer),
}
