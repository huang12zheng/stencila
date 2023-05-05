use crate::prelude::*;

use super::news_media_organization::NewsMediaOrganization;
use super::organization::Organization;

/// For an <a class="localLink" href="/Organization">Organization</a> (e.g. <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>), a statement describing (in news media, the newsroom’s) disclosure and correction policy for errors.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum correctionsPolicy {
    NewsMediaOrganization(NewsMediaOrganization),
    Organization(Organization),
}
