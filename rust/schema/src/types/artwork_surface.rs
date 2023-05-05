use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// The supporting materials for the artwork, e.g. Canvas, Paper, Wood, Board, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum artworkSurface {
    Text(Text),
    URL(URL),
}
