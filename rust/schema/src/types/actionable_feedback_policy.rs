use crate::prelude::*;

use super::news_media_organization::NewsMediaOrganization;
use super::organization::Organization;

/// For a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a> or other news-related <a class="localLink" href="/Organization">Organization</a>, a statement about public engagement activities (for news media, the newsroom’s), including involving the public - digitally or otherwise -- in coverage decisions, reporting and activities after publication.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum actionableFeedbackPolicy {
    NewsMediaOrganization(NewsMediaOrganization),
    Organization(Organization),
}
