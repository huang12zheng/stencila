use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;

/// The date/time at which the message has been read by the recipient if a single recipient exists.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum dateRead {
    Date(Date),
    DateTime(DateTime),
}
