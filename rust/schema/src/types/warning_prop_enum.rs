use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/warning
/// Any FDA or other warnings about the drug (text or URL).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum WarningPropEnum {
    Text(Text),
    URL(URL),
}
