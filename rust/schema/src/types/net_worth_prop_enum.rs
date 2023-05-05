use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::price_specification::PriceSpecification;


/// http://schema.org/netWorth
/// The total financial value of the person as calculated by subtracting assets from liabilities.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum NetWorthPropEnum {
    MonetaryAmount(MonetaryAmount),
    PriceSpecification(PriceSpecification),
}
