use crate::prelude::*;

use super::demand::Demand;
use super::offer::Offer;
use super::price_specification::PriceSpecification;

/// The transaction volume, in a monetary unit, for which the offer or price specification is valid, e.g. for indicating a minimal purchasing volume, to express free shipping above a certain order volume, or to limit the acceptance of credit cards to purchases to a certain minimal amount.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum eligibleTransactionVolume {
    Demand(Demand),
    Offer(Offer),
    PriceSpecification(PriceSpecification),
}
