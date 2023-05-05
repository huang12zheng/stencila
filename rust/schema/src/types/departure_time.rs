use crate::prelude::*;

use super::date_time::DateTime;
use super::time::Time;

/// The expected departure time.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum departureTime {
    DateTime(DateTime),
    Time(Time),
}
