use crate::prelude::*;

use super::number::Number;
use super::text::Text;


/// http://schema.org/latitude
/// The latitude of a location. For example <code>37.42242</code> (<a href="https://en.wikipedia.org/wiki/World_Geodetic_System">WGS 84</a>).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LatitudePropEnum {
    Number(Number),
    Text(Text),
}
