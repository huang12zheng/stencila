use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;

/// For a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>, a statement on coverage priorities, including any public agenda or stance on issues.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum missionCoveragePrioritiesPolicy {
    CreativeWork(CreativeWork),
    URL(URL),
}
