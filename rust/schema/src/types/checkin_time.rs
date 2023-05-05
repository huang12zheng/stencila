use crate::prelude::*;

use super::date_time::DateTime;
use super::time::Time;

/// The earliest someone may check into a lodging establishment.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum checkinTime {
    DateTime(DateTime),
    Time(Time),
}
