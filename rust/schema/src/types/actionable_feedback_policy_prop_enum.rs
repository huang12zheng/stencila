use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;


/// http://schema.org/actionableFeedbackPolicy
/// For a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a> or other news-related <a class="localLink" href="/Organization">Organization</a>, a statement about public engagement activities (for news media, the newsroom’s), including involving the public - digitally or otherwise -- in coverage decisions, reporting and activities after publication.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ActionableFeedbackPolicyPropEnum {
    CreativeWork(CreativeWork),
    URL(URL),
}
