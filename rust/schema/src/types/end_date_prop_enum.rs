use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;


/// http://schema.org/endDate
/// The end date and time of the item (in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a>).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum EndDatePropEnum {
    Date(Date),
    DateTime(DateTime),
}
