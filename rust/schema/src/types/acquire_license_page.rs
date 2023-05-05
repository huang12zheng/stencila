use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;

/// Indicates a page documenting how licenses can be purchased or otherwise acquired, for the current item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum acquireLicensePage {
    CreativeWork(CreativeWork),
    URL(URL),
}
