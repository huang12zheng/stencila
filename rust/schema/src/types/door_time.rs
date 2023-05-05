use crate::prelude::*;

use super::date_time::DateTime;
use super::time::Time;

/// The time admission will commence.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum doorTime {
    DateTime(DateTime),
    Time(Time),
}
