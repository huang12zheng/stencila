use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;

/// Indicates a page (or other CreativeWork) for which this thing is the main entity being described. See <a href="/docs/datamodel.html#mainEntityBackground">background notes</a> for details.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum mainEntityOfPage {
    CreativeWork(CreativeWork),
    URL(URL),
}
