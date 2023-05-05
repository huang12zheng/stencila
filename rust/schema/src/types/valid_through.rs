use crate::prelude::*;

use super::demand::Demand;
use super::job_posting::JobPosting;
use super::location_feature_specification::LocationFeatureSpecification;
use super::monetary_amount::MonetaryAmount;
use super::offer::Offer;
use super::opening_hours_specification::OpeningHoursSpecification;
use super::price_specification::PriceSpecification;

/// The date after when the item is not valid. For example the end of an offer, salary period, or a period of opening hours.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum validThrough {
    Demand(Demand),
    JobPosting(JobPosting),
    LocationFeatureSpecification(LocationFeatureSpecification),
    MonetaryAmount(MonetaryAmount),
    Offer(Offer),
    OpeningHoursSpecification(OpeningHoursSpecification),
    PriceSpecification(PriceSpecification),
}
