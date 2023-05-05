use crate::prelude::*;

use super::demand::Demand;
use super::location_feature_specification::LocationFeatureSpecification;
use super::monetary_amount::MonetaryAmount;
use super::offer::Offer;
use super::opening_hours_specification::OpeningHoursSpecification;
use super::permit::Permit;
use super::price_specification::PriceSpecification;

/// The date when the item becomes valid.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum validFrom {
    Demand(Demand),
    LocationFeatureSpecification(LocationFeatureSpecification),
    MonetaryAmount(MonetaryAmount),
    Offer(Offer),
    OpeningHoursSpecification(OpeningHoursSpecification),
    Permit(Permit),
    PriceSpecification(PriceSpecification),
}
