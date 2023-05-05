use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;
use super::time::Time;

/// The end of the availability of the product or service included in the offer.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum availabilityEnds {
    Date(Date),
    DateTime(DateTime),
    Time(Time),
}
