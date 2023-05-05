use crate::prelude::*;

use super::date_time::DateTime;
use super::time::Time;

/// The latest someone may check out of a lodging establishment.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum checkoutTime {
    DateTime(DateTime),
    Time(Time),
}
