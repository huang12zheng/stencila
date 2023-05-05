use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// Storage requirements (free space required).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum storageRequirements {
    Text(Text),
    URL(URL),
}
