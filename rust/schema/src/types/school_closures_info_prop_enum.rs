use crate::prelude::*;

use super::url::URL;
use super::web_content::WebContent;


/// http://schema.org/schoolClosuresInfo
/// Information about school closures.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SchoolClosuresInfoPropEnum {
    URL(URL),
    WebContent(WebContent),
}
