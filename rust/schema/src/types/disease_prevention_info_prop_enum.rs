use crate::prelude::*;

use super::url::URL;
use super::web_content::WebContent;


/// http://schema.org/diseasePreventionInfo
/// Information about disease prevention.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum DiseasePreventionInfoPropEnum {
    URL(URL),
    WebContent(WebContent),
}