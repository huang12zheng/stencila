use crate::prelude::*;

use super::map::Map;
use super::url::URL;


/// http://schema.org/hasMap
/// A URL to a map of the place.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum HasMapPropEnum {
    Map(Map),
    URL(URL),
}
