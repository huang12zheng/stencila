use crate::prelude::*;

use super::correction_comment::CorrectionComment;
use super::text::Text;
use super::url::URL;

/// Indicates a correction to a <a class="localLink" href="/CreativeWork">CreativeWork</a>, either via a <a class="localLink" href="/CorrectionComment">CorrectionComment</a>, textually or in another document.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum correction {
    CorrectionComment(CorrectionComment),
    Text(Text),
    URL(URL),
}
