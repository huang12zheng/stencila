use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::property_value::PropertyValue;
use super::url::URL;

/// Biological process this BioChemEntity is involved in; please use PropertyValue if you want to include any evidence.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum isInvolvedInBiologicalProcess {
    DefinedTerm(DefinedTerm),
    PropertyValue(PropertyValue),
    URL(URL),
}
