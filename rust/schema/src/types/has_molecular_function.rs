use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::property_value::PropertyValue;
use super::url::URL;

/// Molecular function performed by this BioChemEntity; please use PropertyValue if you want to include any evidence.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum hasMolecularFunction {
    DefinedTerm(DefinedTerm),
    PropertyValue(PropertyValue),
    URL(URL),
}
