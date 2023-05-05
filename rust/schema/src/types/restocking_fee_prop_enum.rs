use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::number::Number;


/// http://schema.org/restockingFee
/// Use <a class="localLink" href="/MonetaryAmount">MonetaryAmount</a> to specify a fixed restocking fee for product returns, or use <a class="localLink" href="/Number">Number</a> to specify a percentage of the product price paid by the customer.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum RestockingFeePropEnum {
    MonetaryAmount(MonetaryAmount),
    Number(Number),
}
