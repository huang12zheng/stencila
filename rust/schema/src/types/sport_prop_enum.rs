use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/sport
/// A type of sport (e.g. Baseball).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SportPropEnum {
    Text(Text),
    URL(URL),
}
