use crate::prelude::*;

use super::date_time::DateTime;
use super::time::Time;


/// http://schema.org/departureTime
/// The expected departure time.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum DepartureTimePropEnum {
    DateTime(DateTime),
    Time(Time),
}
