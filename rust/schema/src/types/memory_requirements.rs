use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// Minimum memory requirements.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum memoryRequirements {
    Text(Text),
    URL(URL),
}
