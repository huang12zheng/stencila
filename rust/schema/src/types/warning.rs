use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// Any FDA or other warnings about the drug (text or URL).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum warning {
    Text(Text),
    URL(URL),
}
