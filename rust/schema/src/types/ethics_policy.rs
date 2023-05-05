use crate::prelude::*;

use super::news_media_organization::NewsMediaOrganization;
use super::organization::Organization;

/// Statement about ethics policy, e.g. of a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a> regarding journalistic and publishing practices, or of a <a class="localLink" href="/Restaurant">Restaurant</a>, a page describing food source policies. In the case of a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>, an ethicsPolicy is typically a statement describing the personal, organizational, and corporate standards of behavior expected by the organization.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum ethicsPolicy {
    NewsMediaOrganization(NewsMediaOrganization),
    Organization(Organization),
}
