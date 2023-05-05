use crate::prelude::*;

use super::correction_comment::CorrectionComment;
use super::text::Text;
use super::url::URL;


/// http://schema.org/correction
/// Indicates a correction to a <a class="localLink" href="/CreativeWork">CreativeWork</a>, either via a <a class="localLink" href="/CorrectionComment">CorrectionComment</a>, textually or in another document.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CorrectionPropEnum {
    CorrectionComment(CorrectionComment),
    Text(Text),
    URL(URL),
}
