use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;


/// http://schema.org/documentation
/// Further documentation describing the Web API in more detail.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum DocumentationPropEnum {
    CreativeWork(CreativeWork),
    URL(URL),
}
