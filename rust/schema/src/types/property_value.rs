// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::measurement_technique_prop_enum::MeasurementTechniquePropEnum;
use super::number::Number;
use super::property_id_prop_enum::PropertyIDPropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;
use super::unit_code_prop_enum::UnitCodePropEnum;
use super::value_prop_enum::ValuePropEnum;
use super::value_reference_prop_enum::ValueReferencePropEnum;

/// https://schema.org/PropertyValue
/// * COMMENT:
/// A property-value pair, e.g. representing a feature of a product or place. Use the 'name' property for the name of the property. If there is an additional human-readable version of the value, put that into the 'description' property.<br/><br/>
/// 
/// Always use specific schema.org properties when a) they exist and b) you can populate them. Using PropertyValue as a substitute will typically not trigger the same effect as using the original, specific property.
/// * EXTEND FROM:
/// https://schema.org/StructuredValue
/// * LOOK ALSO:
/// https://schema.org/LocationFeatureSpecification
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct PropertyValue {
    /// An additional type for the item, typically used for adding more specific types from external vocabularies in microdata syntax. This is a relationship between something and a class that the thing is in. In RDFa syntax, it is better to use the native RDFa syntax - the 'typeof' attribute - for multiple types. Schema.org tools may have only weaker understanding of extra types, in particular those defined externally.
    pub additional_type_prop_enum: Option<URL>,

    /// An alias for the item.
    pub alternate_name_prop_enum: Option<Text>,

    /// A description of the item.
    pub description_prop_enum: Option<Text>,

    /// A sub property of description. A short description of the item used to disambiguate from other, similar items. Information from other properties (in particular, name) may be necessary for the description to be useful for disambiguation.
    pub disambiguating_description_prop_enum: Option<Text>,

    /// The identifier property represents any kind of identifier for any kind of <a class="localLink" href="/Thing">Thing</a>, such as ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing many of these, either as textual strings or as URL (URI) links. See <a href="/docs/datamodel.html#identifierBg">background notes</a> for more details.
    pub identifier_prop_enum: Option<IdentifierPropEnum>,

    /// An image of the item. This can be a <a class="localLink" href="/URL">URL</a> or a fully described <a class="localLink" href="/ImageObject">ImageObject</a>.
    pub image_prop_enum: Option<ImagePropEnum>,

    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being described. See <a href="/docs/datamodel.html#mainEntityBackground">background notes</a> for details.
    pub main_entity_of_page_prop_enum: Option<MainEntityOfPagePropEnum>,

    /// The name of the item.
    pub name_prop_enum: Option<Text>,

    /// Indicates a potential Action, which describes an idealized action in which this thing would play an 'object' role.
    pub potential_action_prop_enum: Option<Action>,

    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL of the item's Wikipedia page, Wikidata entry, or official website.
    pub same_as_prop_enum: Option<URL>,

    /// A CreativeWork or Event about this Thing.
    pub subject_of_prop_enum: Option<SubjectOfPropEnum>,

    /// URL of the item.
    pub url_prop_enum: Option<URL>,

    /// The upper value of some characteristic or property.
    pub max_value_prop_enum: Option<Number>,

    /// A technique or technology used in a <a class="localLink" href="/Dataset">Dataset</a> (or <a class="localLink" href="/DataDownload">DataDownload</a>, <a class="localLink" href="/DataCatalog">DataCatalog</a>), corresponding to the method used for measuring the corresponding variable(s) (described using <a class="localLink" href="/variableMeasured">variableMeasured</a>). This is oriented towards scientific and scholarly dataset publication but may have broader applicability; it is not intended as a full representation of measurement, but rather as a high level summary for dataset discovery.<br/><br/>  For example, if <a class="localLink" href="/variableMeasured">variableMeasured</a> is: molecule concentration, <a class="localLink" href="/measurementTechnique">measurementTechnique</a> could be: "mass spectrometry" or "nmr spectroscopy" or "colorimetry" or "immunofluorescence".<br/><br/>  If the <a class="localLink" href="/variableMeasured">variableMeasured</a> is "depression rating", the <a class="localLink" href="/measurementTechnique">measurementTechnique</a> could be "Zung Scale" or "HAM-D" or "Beck Depression Inventory".<br/><br/>  If there are several <a class="localLink" href="/variableMeasured">variableMeasured</a> properties recorded for some given data object, use a <a class="localLink" href="/PropertyValue">PropertyValue</a> for each <a class="localLink" href="/variableMeasured">variableMeasured</a> and attach the corresponding <a class="localLink" href="/measurementTechnique">measurementTechnique</a>.
    pub measurement_technique_prop_enum: Option<MeasurementTechniquePropEnum>,

    /// The lower value of some characteristic or property.
    pub min_value_prop_enum: Option<Number>,

    /// A commonly used identifier for the characteristic represented by the property, e.g. a manufacturer or a standard code for a property. propertyID can be (1) a prefixed string, mainly meant to be used with standards for product properties; (2) a site-specific, non-prefixed string (e.g. the primary key of the property or the vendor-specific ID of the property), or (3) a URL indicating the type of the property, either pointing to an external vocabulary, or a Web resource that describes the property (e.g. a glossary entry). Standards bodies should promote a standard prefix for the identifiers of properties from their standards.
    pub property_id_prop_enum: Option<PropertyIDPropEnum>,

    /// The unit of measurement given using the UN/CEFACT Common Code (3 characters) or a URL. Other codes than the UN/CEFACT Common Code may be used with a prefix followed by a colon.
    pub unit_code_prop_enum: Option<UnitCodePropEnum>,

    /// A string or text indicating the unit of measurement. Useful if you cannot provide a standard unit code for <a href='unitCode'>unitCode</a>.
    pub unit_text_prop_enum: Option<Text>,

    /// The value of the quantitative value or property value node.<br/><br/>  <ul> <li>For <a class="localLink" href="/QuantitativeValue">QuantitativeValue</a> and <a class="localLink" href="/MonetaryAmount">MonetaryAmount</a>, the recommended type for values is 'Number'.</li> <li>For <a class="localLink" href="/PropertyValue">PropertyValue</a>, it can be 'Text', 'Number', 'Boolean', or 'StructuredValue'.</li> <li>Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similar Unicode symbols.</li> <li>Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.</li> </ul>
    pub value_prop_enum: Option<ValuePropEnum>,

    /// A secondary value that provides additional information on the original value, e.g. a reference temperature or a type of measurement.
    pub value_reference_prop_enum: Option<ValueReferencePropEnum>,
}

impl PropertyValue {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
