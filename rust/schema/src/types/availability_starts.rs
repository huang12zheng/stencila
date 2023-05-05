use crate::prelude::*;

use super::action_access_specification::ActionAccessSpecification;
use super::demand::Demand;
use super::offer::Offer;

/// The beginning of the availability of the product or service included in the offer.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum availabilityStarts {
    ActionAccessSpecification(ActionAccessSpecification),
    Demand(Demand),
    Offer(Offer),
}
