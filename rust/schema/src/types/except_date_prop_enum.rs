use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;


/// http://schema.org/exceptDate
/// Defines a <a class="localLink" href="/Date">Date</a> or <a class="localLink" href="/DateTime">DateTime</a> during which a scheduled <a class="localLink" href="/Event">Event</a> will not take place. The property allows exceptions to       a <a class="localLink" href="/Schedule">Schedule</a> to be specified. If an exception is specified as a <a class="localLink" href="/DateTime">DateTime</a> then only the event that would have started at that specific date and time       should be excluded from the schedule. If an exception is specified as a <a class="localLink" href="/Date">Date</a> then any event that is scheduled for that 24 hour period should be       excluded from the schedule. This allows a whole day to be excluded from the schedule without having to itemise every scheduled event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ExceptDatePropEnum {
    Date(Date),
    DateTime(DateTime),
}
