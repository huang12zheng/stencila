use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// A type of sport (e.g. Baseball).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum sport {
    Text(Text),
    URL(URL),
}
