use crate::prelude::*;

use super::date_time::DateTime;
use super::time::Time;

/// The expected arrival time.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum arrivalTime {
    DateTime(DateTime),
    Time(Time),
}
