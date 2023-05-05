use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;

/// The date after when the item is not valid. For example the end of an offer, salary period, or a period of opening hours.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum validThrough {
    Date(Date),
    DateTime(DateTime),
}
