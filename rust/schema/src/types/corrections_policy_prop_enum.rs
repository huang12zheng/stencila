use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;


/// http://schema.org/correctionsPolicy
/// For an <a class="localLink" href="/Organization">Organization</a> (e.g. <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>), a statement describing (in news media, the newsroom’s) disclosure and correction policy for errors.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CorrectionsPolicyPropEnum {
    CreativeWork(CreativeWork),
    URL(URL),
}
