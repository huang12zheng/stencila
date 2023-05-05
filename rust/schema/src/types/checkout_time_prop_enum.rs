use crate::prelude::*;

use super::date_time::DateTime;
use super::time::Time;


/// http://schema.org/checkoutTime
/// The latest someone may check out of a lodging establishment.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CheckoutTimePropEnum {
    DateTime(DateTime),
    Time(Time),
}
