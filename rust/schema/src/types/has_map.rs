use crate::prelude::*;

use super::map::Map;
use super::url::URL;

/// A URL to a map of the place.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum hasMap {
    Map(Map),
    URL(URL),
}
