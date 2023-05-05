use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;


/// http://schema.org/unnamedSourcesPolicy
/// For an <a class="localLink" href="/Organization">Organization</a> (typically a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>), a statement about policy on use of unnamed sources and the decision process required.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum UnnamedSourcesPolicyPropEnum {
    CreativeWork(CreativeWork),
    URL(URL),
}
