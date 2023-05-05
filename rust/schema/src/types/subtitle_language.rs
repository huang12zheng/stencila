use crate::prelude::*;

use super::language::Language;
use super::text::Text;

/// Languages in which subtitles/captions are available, in <a href="http://tools.ietf.org/html/bcp47">IETF BCP 47 standard format</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum subtitleLanguage {
    Language(Language),
    Text(Text),
}
