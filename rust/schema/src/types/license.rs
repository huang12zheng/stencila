use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;

/// A license document that applies to this content, typically indicated by URL.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum license {
    CreativeWork(CreativeWork),
    URL(URL),
}
