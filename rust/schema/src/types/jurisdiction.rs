use crate::prelude::*;

use super::administrative_area::AdministrativeArea;
use super::text::Text;

/// Indicates a legal jurisdiction, e.g. of some legislation, or where some government service is based.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum jurisdiction {
    AdministrativeArea(AdministrativeArea),
    Text(Text),
}
