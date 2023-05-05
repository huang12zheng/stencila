use crate::prelude::*;

use super::medical_condition::MedicalCondition;
use super::property_value::PropertyValue;
use super::url::URL;


/// http://schema.org/associatedDisease
/// Disease associated to this BioChemEntity. Such disease can be a MedicalCondition or a URL. If you want to add an evidence supporting the association, please use PropertyValue.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AssociatedDiseasePropEnum {
    MedicalCondition(MedicalCondition),
    PropertyValue(PropertyValue),
    URL(URL),
}
