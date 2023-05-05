use crate::prelude::*;

use super::administrative_area::AdministrativeArea;
use super::text::Text;


/// http://schema.org/jurisdiction
/// Indicates a legal jurisdiction, e.g. of some legislation, or where some government service is based.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum JurisdictionPropEnum {
    AdministrativeArea(AdministrativeArea),
    Text(Text),
}
