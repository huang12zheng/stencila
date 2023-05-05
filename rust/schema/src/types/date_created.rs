use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;

/// The date on which the CreativeWork was created or the item was added to a DataFeed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum dateCreated {
    Date(Date),
    DateTime(DateTime),
}
