use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;

/// Indicates an item or CreativeWork that this item, or CreativeWork (in some sense), is part of.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum isPartOf {
    CreativeWork(CreativeWork),
    URL(URL),
}
