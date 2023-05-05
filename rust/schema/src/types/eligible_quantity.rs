use crate::prelude::*;

use super::demand::Demand;
use super::offer::Offer;
use super::price_specification::PriceSpecification;

/// The interval and unit of measurement of ordering quantities for which the offer or price specification is valid. This allows e.g. specifying that a certain freight charge is valid only for a certain quantity.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum eligibleQuantity {
    Demand(Demand),
    Offer(Offer),
    PriceSpecification(PriceSpecification),
}
