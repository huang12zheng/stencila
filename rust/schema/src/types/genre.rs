use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// Genre of the creative work, broadcast channel or group.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum genre {
    Text(Text),
    URL(URL),
}
