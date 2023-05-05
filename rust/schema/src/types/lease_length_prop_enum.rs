use crate::prelude::*;

use super::duration::Duration;
use super::quantitative_value::QuantitativeValue;


/// http://schema.org/leaseLength
/// Length of the lease for some <a class="localLink" href="/Accommodation">Accommodation</a>, either particular to some <a class="localLink" href="/Offer">Offer</a> or in some cases intrinsic to the property.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LeaseLengthPropEnum {
    Duration(Duration),
    QuantitativeValue(QuantitativeValue),
}
