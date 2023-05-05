use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;

/// The latest date the package may arrive.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum expectedArrivalUntil {
    Date(Date),
    DateTime(DateTime),
}
