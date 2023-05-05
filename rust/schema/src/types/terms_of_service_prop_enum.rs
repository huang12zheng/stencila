use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/termsOfService
/// Human-readable terms of service documentation.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum TermsOfServicePropEnum {
    Text(Text),
    URL(URL),
}
