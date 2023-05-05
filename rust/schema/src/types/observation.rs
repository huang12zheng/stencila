// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::data_type::DataType;
use super::date_time::DateTime;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::property::Property;
use super::quantitative_value::QuantitativeValue;
use super::statistical_population::StatisticalPopulation;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;

/// https://schema.org/Observation
/// * MOD OF:
/// https://pending.schema.org
/// * COMMENT:
/// Instances of the class <a class="localLink" href="/Observation">Observation</a> are used to specify observations about an entity (which may or may not be an instance of a <a class="localLink" href="/StatisticalPopulation">StatisticalPopulation</a>), at a particular time. The principal properties of an <a class="localLink" href="/Observation">Observation</a> are <a class="localLink" href="/observedNode">observedNode</a>, <a class="localLink" href="/measuredProperty">measuredProperty</a>, <a class="localLink" href="/measuredValue">measuredValue</a> (or <a class="localLink" href="/median">median</a>, etc.) and <a class="localLink" href="/observationDate">observationDate</a> (<a class="localLink" href="/measuredProperty">measuredProperty</a> properties can, but need not always, be W3C RDF Data Cube "measure properties", as in the <a href="https://www.w3.org/TR/vocab-data-cube/#dsd-example">lifeExpectancy example</a>).
/// See also <a class="localLink" href="/StatisticalPopulation">StatisticalPopulation</a>, and the <a href="/docs/data-and-datasets.html">data and datasets</a> overview for more details.
/// * EXTEND FROM:
/// https://schema.org/Intangible
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Observation {
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

    /// A marginOfError for an <a class="localLink" href="/Observation">Observation</a>.
    pub margin_of_error_prop_enum: Option<QuantitativeValue>,

    /// The measuredProperty of an <a class="localLink" href="/Observation">Observation</a>, either a schema.org property, a property from other RDF-compatible systems, e.g. W3C RDF Data Cube, or schema.org extensions such as <a href="https://www.gs1.org/voc/?show=properties">GS1's</a>.
    pub measured_property_prop_enum: Option<Property>,

    /// The measuredValue of an <a class="localLink" href="/Observation">Observation</a>.
    pub measured_value_prop_enum: Option<DataType>,

    /// The observationDate of an <a class="localLink" href="/Observation">Observation</a>.
    pub observation_date_prop_enum: Option<DateTime>,

    /// The observedNode of an <a class="localLink" href="/Observation">Observation</a>, often a <a class="localLink" href="/StatisticalPopulation">StatisticalPopulation</a>.
    pub observed_node_prop_enum: Option<StatisticalPopulation>,
}

impl Observation {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
