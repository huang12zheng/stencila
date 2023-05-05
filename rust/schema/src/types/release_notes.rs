use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// Description of what changed in this version.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum releaseNotes {
    Text(Text),
    URL(URL),
}
