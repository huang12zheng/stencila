use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// Features or modules provided by this application (and possibly required by other applications).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum featureList {
    Text(Text),
    URL(URL),
}
