use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::enumeration::Enumeration;
use super::measurement_type_enumeration::MeasurementTypeEnumeration;
use super::property_value::PropertyValue;
use super::qualitative_value::QualitativeValue;
use super::quantitative_value::QuantitativeValue;
use super::structured_value::StructuredValue;
use super::text::Text;


/// http://schema.org/valueReference
/// A secondary value that provides additional information on the original value, e.g. a reference temperature or a type of measurement.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ValueReferencePropEnum {
    DefinedTerm(DefinedTerm),
    Enumeration(Enumeration),
    MeasurementTypeEnumeration(MeasurementTypeEnumeration),
    PropertyValue(PropertyValue),
    QualitativeValue(QualitativeValue),
    QuantitativeValue(QuantitativeValue),
    StructuredValue(StructuredValue),
    Text(Text),
}
