use crate::prelude::*;

use super::url::URL;
use super::web_content::WebContent;


/// http://schema.org/gettingTestedInfo
/// Information about getting tested (for a <a class="localLink" href="/MedicalCondition">MedicalCondition</a>), e.g. in the context of a pandemic.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum GettingTestedInfoPropEnum {
    URL(URL),
    WebContent(WebContent),
}