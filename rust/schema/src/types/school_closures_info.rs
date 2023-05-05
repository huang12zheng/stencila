use crate::prelude::*;

use super::url::URL;
use super::web_content::WebContent;

/// Information about school closures.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum schoolClosuresInfo {
    URL(URL),
    WebContent(WebContent),
}
