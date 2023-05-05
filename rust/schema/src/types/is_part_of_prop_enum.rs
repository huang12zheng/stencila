use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;


/// http://schema.org/isPartOf
/// Indicates an item or CreativeWork that this item, or CreativeWork (in some sense), is part of.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum IsPartOfPropEnum {
    CreativeWork(CreativeWork),
    URL(URL),
}
