use crate::prelude::*;

use super::news_media_organization::NewsMediaOrganization;
use super::organization::Organization;

/// For an <a class="localLink" href="/Organization">Organization</a> (typically a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>), a statement about policy on use of unnamed sources and the decision process required.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum unnamedSourcesPolicy {
    NewsMediaOrganization(NewsMediaOrganization),
    Organization(Organization),
}
