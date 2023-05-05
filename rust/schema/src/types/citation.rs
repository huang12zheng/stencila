use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::text::Text;

/// A citation or reference to another creative work, such as another publication, web page, scholarly article, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum citation {
    CreativeWork(CreativeWork),
    Text(Text),
}
