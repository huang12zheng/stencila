use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;

/// The start date and time of the item (in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a>).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum startDate {
    Date(Date),
    DateTime(DateTime),
}
