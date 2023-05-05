use crate::prelude::*;

use super::bed_type::BedType;
use super::text::Text;

/// The type of bed to which the BedDetail refers, i.e. the type of bed available in the quantity indicated by quantity.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum typeOfBed {
    BedType(BedType),
    Text(Text),
}
