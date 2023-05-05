use crate::prelude::*;

use super::date_time::DateTime;
use super::text::Text;

/// collectiondate - Date for which patient counts are reported.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum cvdCollectionDate {
    DateTime(DateTime),
    Text(Text),
}
