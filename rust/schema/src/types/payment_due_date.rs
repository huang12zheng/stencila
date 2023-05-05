use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;

/// The date that payment is due.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum paymentDueDate {
    Date(Date),
    DateTime(DateTime),
}
