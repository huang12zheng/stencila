use crate::prelude::*;

use super::country::Country;
use super::text::Text;

/// A country where a particular merchant return policy applies to, for example the two-letter ISO 3166-1 alpha-2 country code.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum applicableCountry {
    Country(Country),
    Text(Text),
}
