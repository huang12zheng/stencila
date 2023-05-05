use crate::prelude::*;

use super::administrative_area::AdministrativeArea;
use super::text::Text;


/// http://schema.org/legislationJurisdiction
/// The jurisdiction from which the legislation originates.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LegislationJurisdictionPropEnum {
    AdministrativeArea(AdministrativeArea),
    Text(Text),
}
