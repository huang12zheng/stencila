use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;
use super::time::Time;


/// http://schema.org/availabilityStarts
/// The beginning of the availability of the product or service included in the offer.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AvailabilityStartsPropEnum {
    Date(Date),
    DateTime(DateTime),
    Time(Time),
}
