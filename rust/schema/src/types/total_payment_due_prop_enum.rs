use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::price_specification::PriceSpecification;


/// http://schema.org/totalPaymentDue
/// The total amount due.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum TotalPaymentDuePropEnum {
    MonetaryAmount(MonetaryAmount),
    PriceSpecification(PriceSpecification),
}
