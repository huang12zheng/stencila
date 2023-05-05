use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::hyper_toc::HyperToc;
use super::hyper_toc_entry::HyperTocEntry;

/// A media object that encodes this CreativeWork. This property is a synonym for encoding.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum associatedMedia {
    CreativeWork(CreativeWork),
    HyperToc(HyperToc),
    HyperTocEntry(HyperTocEntry),
}
