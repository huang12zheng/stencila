use crate::prelude::*;

use super::language::Language;
use super::text::Text;


/// http://schema.org/subtitleLanguage
/// Languages in which subtitles/captions are available, in <a href="http://tools.ietf.org/html/bcp47">IETF BCP 47 standard format</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SubtitleLanguagePropEnum {
    Language(Language),
    Text(Text),
}
