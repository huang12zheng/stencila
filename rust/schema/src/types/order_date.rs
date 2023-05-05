use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;

/// Date order was placed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum orderDate {
    Date(Date),
    DateTime(DateTime),
}
