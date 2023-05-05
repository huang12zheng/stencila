use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// Human-readable terms of service documentation.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum termsOfService {
    Text(Text),
    URL(URL),
}
