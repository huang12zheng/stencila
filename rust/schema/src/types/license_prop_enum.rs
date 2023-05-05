use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;


/// http://schema.org/license
/// A license document that applies to this content, typically indicated by URL.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LicensePropEnum {
    CreativeWork(CreativeWork),
    URL(URL),
}
