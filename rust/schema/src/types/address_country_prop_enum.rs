use crate::prelude::*;

use super::country::Country;
use super::text::Text;


/// http://schema.org/addressCountry
/// The country. For example, USA. You can also provide the two-letter <a href="http://en.wikipedia.org/wiki/ISO_3166-1">ISO 3166-1 alpha-2 country code</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AddressCountryPropEnum {
    Country(Country),
    Text(Text),
}
