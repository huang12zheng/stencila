use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;


/// http://schema.org/dateRead
/// The date/time at which the message has been read by the recipient if a single recipient exists.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum DateReadPropEnum {
    Date(Date),
    DateTime(DateTime),
}
