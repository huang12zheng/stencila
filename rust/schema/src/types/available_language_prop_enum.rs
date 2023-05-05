use crate::prelude::*;

use super::language::Language;
use super::text::Text;


/// http://schema.org/availableLanguage
/// A language someone may use with or at the item, service or place. Please use one of the language codes from the <a href="http://tools.ietf.org/html/bcp47">IETF BCP 47 standard</a>. See also <a class="localLink" href="/inLanguage">inLanguage</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AvailableLanguagePropEnum {
    Language(Language),
    Text(Text),
}
