use crate::prelude::*;

use super::date_time::DateTime;
use super::time::Time;


/// http://schema.org/checkinTime
/// The earliest someone may check into a lodging establishment.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CheckinTimePropEnum {
    DateTime(DateTime),
    Time(Time),
}