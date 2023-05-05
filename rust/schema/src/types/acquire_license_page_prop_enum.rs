use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;


/// http://schema.org/acquireLicensePage
/// Indicates a page documenting how licenses can be purchased or otherwise acquired, for the current item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AcquireLicensePagePropEnum {
    CreativeWork(CreativeWork),
    URL(URL),
}
