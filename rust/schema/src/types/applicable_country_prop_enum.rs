use crate::prelude::*;

use super::country::Country;
use super::text::Text;


/// http://schema.org/applicableCountry
/// A country where a particular merchant return policy applies to, for example the two-letter ISO 3166-1 alpha-2 country code.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ApplicableCountryPropEnum {
    Country(Country),
    Text(Text),
}
