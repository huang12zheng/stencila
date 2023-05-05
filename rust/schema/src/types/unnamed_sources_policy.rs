use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;

/// For an <a class="localLink" href="/Organization">Organization</a> (typically a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>), a statement about policy on use of unnamed sources and the decision process required.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum unnamedSourcesPolicy {
    CreativeWork(CreativeWork),
    URL(URL),
}
