use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;

/// Further documentation describing the Web API in more detail.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum documentation {
    CreativeWork(CreativeWork),
    URL(URL),
}
