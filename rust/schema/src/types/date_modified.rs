use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;

/// The date on which the CreativeWork was most recently modified or when the item's entry was modified within a DataFeed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum dateModified {
    Date(Date),
    DateTime(DateTime),
}
