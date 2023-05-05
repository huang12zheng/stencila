use crate::prelude::*;

use super::administrative_area::AdministrativeArea;
use super::text::Text;

/// The jurisdiction from which the legislation originates.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum legislationJurisdiction {
    AdministrativeArea(AdministrativeArea),
    Text(Text),
}
