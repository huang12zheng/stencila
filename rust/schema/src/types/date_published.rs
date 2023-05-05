use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;

/// Date of first broadcast/publication.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum datePublished {
    Date(Date),
    DateTime(DateTime),
}
