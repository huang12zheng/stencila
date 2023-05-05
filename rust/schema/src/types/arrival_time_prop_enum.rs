use crate::prelude::*;

use super::date_time::DateTime;
use super::time::Time;


/// http://schema.org/arrivalTime
/// The expected arrival time.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ArrivalTimePropEnum {
    DateTime(DateTime),
    Time(Time),
}
