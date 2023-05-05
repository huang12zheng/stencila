// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::in_language::inLanguage;

/// * MOD OF: https://pending.schema.org * COMMENT: Data type: PronounceableText. * EXTEND FROM: https://schema.org/Text
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct PronounceableText {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<PronounceableTextOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct PronounceableTextOptions {
    /// The language of the content or performance or used in an action. Please use one of the language codes from the <a href="http://tools.ietf.org/html/bcp47">IETF BCP 47 standard</a>. See also <a class="localLink" href="/availableLanguage">availableLanguage</a>.
    pub in_language: Option<inLanguage>,

    /// Representation of a text <a class="localLink" href="/textValue">textValue</a> using the specified <a class="localLink" href="/speechToTextMarkup">speechToTextMarkup</a>. For example the city name of Houston in IPA: /ˈhjuːstən/.
    pub phonetic_text: Option<PronounceableText>,

    /// Form of markup used. eg. <a href="https://www.w3.org/TR/speech-synthesis11">SSML</a> or <a href="https://www.wikidata.org/wiki/Property:P898">IPA</a>.
    pub speech_to_text_markup: Option<PronounceableText>,

    /// Text value being annotated.
    pub text_value: Option<PronounceableText>,
}

impl PronounceableText {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
