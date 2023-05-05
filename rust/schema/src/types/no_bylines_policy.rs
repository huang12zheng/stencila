use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;

/// For a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a> or other news-related <a class="localLink" href="/Organization">Organization</a>, a statement explaining when authors of articles are not named in bylines.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum noBylinesPolicy {
    CreativeWork(CreativeWork),
    URL(URL),
}
