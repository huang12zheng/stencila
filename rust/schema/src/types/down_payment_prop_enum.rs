use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::number::Number;


/// http://schema.org/downPayment
/// a type of payment made in cash during the onset of the purchase of an expensive good/service. The payment typically represents only a percentage of the full purchase price.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum DownPaymentPropEnum {
    MonetaryAmount(MonetaryAmount),
    Number(Number),
}
