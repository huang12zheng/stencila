use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::price_specification::PriceSpecification;

/// The minimum payment required at this time.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum minimumPaymentDue {
    MonetaryAmount(MonetaryAmount),
    PriceSpecification(PriceSpecification),
}
