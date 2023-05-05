use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;

/// The date when the item becomes valid.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum validFrom {
    Date(Date),
    DateTime(DateTime),
}
