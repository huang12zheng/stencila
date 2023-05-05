use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;

/// For a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>, a link to the masthead page or a page listing top editorial management.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum masthead {
    CreativeWork(CreativeWork),
    URL(URL),
}
