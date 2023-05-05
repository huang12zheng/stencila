use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::property_value::PropertyValue;
use super::url::URL;


/// http://schema.org/isLocatedInSubcellularLocation
/// Subcellular location where this BioChemEntity is located; please use PropertyValue if you want to include any evidence.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum IsLocatedInSubcellularLocationPropEnum {
    DefinedTerm(DefinedTerm),
    PropertyValue(PropertyValue),
    URL(URL),
}
