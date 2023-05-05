use crate::prelude::*;

use super::news_media_organization::NewsMediaOrganization;
use super::organization::Organization;

/// Statement on diversity policy by an <a class="localLink" href="/Organization">Organization</a> e.g. a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>. For a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>, a statement describing the newsroom’s diversity policy on both staffing and sources, typically providing staffing data.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum diversityPolicy {
    NewsMediaOrganization(NewsMediaOrganization),
    Organization(Organization),
}
