use crate::prelude::*;

use super::language::Language;
use super::text::Text;


/// http://schema.org/inLanguage
/// The language of the content or performance or used in an action. Please use one of the language codes from the <a href="http://tools.ietf.org/html/bcp47">IETF BCP 47 standard</a>. See also <a class="localLink" href="/availableLanguage">availableLanguage</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum InLanguagePropEnum {
    Language(Language),
    Text(Text),
}
