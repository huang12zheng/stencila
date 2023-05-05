use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::property_value::PropertyValue;
use super::url::URL;


/// http://schema.org/isInvolvedInBiologicalProcess
/// Biological process this BioChemEntity is involved in; please use PropertyValue if you want to include any evidence.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum IsInvolvedInBiologicalProcessPropEnum {
    DefinedTerm(DefinedTerm),
    PropertyValue(PropertyValue),
    URL(URL),
}
