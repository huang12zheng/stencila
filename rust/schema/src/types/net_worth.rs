use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::price_specification::PriceSpecification;

/// The total financial value of the person as calculated by subtracting assets from liabilities.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum netWorth {
    MonetaryAmount(MonetaryAmount),
    PriceSpecification(PriceSpecification),
}
