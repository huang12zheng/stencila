use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;


/// http://schema.org/missionCoveragePrioritiesPolicy
/// For a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>, a statement on coverage priorities, including any public agenda or stance on issues.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum MissionCoveragePrioritiesPolicyPropEnum {
    CreativeWork(CreativeWork),
    URL(URL),
}
