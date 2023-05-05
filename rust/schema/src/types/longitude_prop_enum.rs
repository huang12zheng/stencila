use crate::prelude::*;

use super::number::Number;
use super::text::Text;


/// http://schema.org/longitude
/// The longitude of a location. For example <code>-122.08585</code> (<a href="https://en.wikipedia.org/wiki/World_Geodetic_System">WGS 84</a>).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LongitudePropEnum {
    Number(Number),
    Text(Text),
}
