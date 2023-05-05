use crate::prelude::*;

use super::size_system_enumeration::SizeSystemEnumeration;
use super::text::Text;


/// http://schema.org/sizeSystem
/// The size system used to identify a product's size. Typically either a standard (for example, "GS1" or "ISO-EN13402"), country code (for example "US" or "JP"), or a measuring system (for example "Metric" or "Imperial").
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SizeSystemPropEnum {
    SizeSystemEnumeration(SizeSystemEnumeration),
    Text(Text),
}
