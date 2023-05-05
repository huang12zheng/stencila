use crate::prelude::*;

use super::bed_type::BedType;
use super::text::Text;


/// http://schema.org/typeOfBed
/// The type of bed to which the BedDetail refers, i.e. the type of bed available in the quantity indicated by quantity.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum TypeOfBedPropEnum {
    BedType(BedType),
    Text(Text),
}
