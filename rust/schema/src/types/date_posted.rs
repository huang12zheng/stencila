use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;

/// Publication date of an online listing.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum datePosted {
    Date(Date),
    DateTime(DateTime),
}
