use crate::prelude::*;

use super::audience::Audience;
use super::text::Text;


/// http://schema.org/touristType
/// Attraction suitable for type(s) of tourist. E.g. children, visitors from a particular country, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum TouristTypePropEnum {
    Audience(Audience),
    Text(Text),
}
