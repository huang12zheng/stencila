use crate::prelude::*;

use super::medical_condition::MedicalCondition;
use super::property_value::PropertyValue;
use super::url::URL;

/// Disease associated to this BioChemEntity. Such disease can be a MedicalCondition or a URL. If you want to add an evidence supporting the association, please use PropertyValue.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum associatedDisease {
    MedicalCondition(MedicalCondition),
    PropertyValue(PropertyValue),
    URL(URL),
}
