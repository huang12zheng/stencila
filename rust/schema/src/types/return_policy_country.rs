use crate::prelude::*;

use super::country::Country;
use super::text::Text;

/// The country where the product has to be sent to for returns, for example "Ireland" using the <a class="localLink" href="/name">name</a> property of <a class="localLink" href="/Country">Country</a>. You can also provide the two-letter <a href="http://en.wikipedia.org/wiki/ISO_3166-1">ISO 3166-1 alpha-2 country code</a>. Note that this can be different from the country where the product was originally shipped from or sent to.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum returnPolicyCountry {
    Country(Country),
    Text(Text),
}
