use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::enumeration::Enumeration;
use super::measurement_type_enumeration::MeasurementTypeEnumeration;
use super::property_value::PropertyValue;
use super::qualitative_value::QualitativeValue;
use super::quantitative_value::QuantitativeValue;
use super::structured_value::StructuredValue;
use super::text::Text;

/// A secondary value that provides additional information on the original value, e.g. a reference temperature or a type of measurement.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum valueReference {
    DefinedTerm(DefinedTerm),
    Enumeration(Enumeration),
    MeasurementTypeEnumeration(MeasurementTypeEnumeration),
    PropertyValue(PropertyValue),
    QualitativeValue(QualitativeValue),
    QuantitativeValue(QuantitativeValue),
    StructuredValue(StructuredValue),
    Text(Text),
}
