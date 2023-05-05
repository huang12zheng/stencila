use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::price_specification::PriceSpecification;

/// The total amount due.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum totalPaymentDue {
    MonetaryAmount(MonetaryAmount),
    PriceSpecification(PriceSpecification),
}
