use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;

/// The date the ticket was issued.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum dateIssued {
    Date(Date),
    DateTime(DateTime),
}
