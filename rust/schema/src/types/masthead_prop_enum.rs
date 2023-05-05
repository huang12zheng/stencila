use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;


/// http://schema.org/masthead
/// For a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>, a link to the masthead page or a page listing top editorial management.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum MastheadPropEnum {
    CreativeWork(CreativeWork),
    URL(URL),
}
