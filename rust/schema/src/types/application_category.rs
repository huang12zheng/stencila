use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// Type of software application, e.g. 'Game, Multimedia'.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum applicationCategory {
    Text(Text),
    URL(URL),
}
