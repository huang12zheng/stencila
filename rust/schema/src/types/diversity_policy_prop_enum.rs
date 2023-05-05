use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;


/// http://schema.org/diversityPolicy
/// Statement on diversity policy by an <a class="localLink" href="/Organization">Organization</a> e.g. a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>. For a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>, a statement describing the newsroom’s diversity policy on both staffing and sources, typically providing staffing data.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum DiversityPolicyPropEnum {
    CreativeWork(CreativeWork),
    URL(URL),
}
