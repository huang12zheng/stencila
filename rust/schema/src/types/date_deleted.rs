use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;

/// The datetime the item was removed from the DataFeed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum dateDeleted {
    Date(Date),
    DateTime(DateTime),
}
