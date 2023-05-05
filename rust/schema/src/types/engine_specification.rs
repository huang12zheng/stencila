// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::thing::Thing;
use super::fuel_type::fuelType;

/// * COMMENT: Information about the engine of the vehicle. A vehicle can have multiple engines represented by multiple engine specification entities. * EXTEND FROM: https://schema.org/StructuredValue
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct EngineSpecification {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<EngineSpecificationOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct EngineSpecificationOptions {
    /// An additional type for the item, typically used for adding more specific types from external vocabularies in microdata syntax. This is a relationship between something and a class that the thing is in. In RDFa syntax, it is better to use the native RDFa syntax - the 'typeof' attribute - for multiple types. Schema.org tools may have only weaker understanding of extra types, in particular those defined externally.
    pub additional_type: Option<Thing>,

    /// An alias for the item.
    pub alternate_name: Option<Thing>,

    /// A description of the item.
    pub description: Option<Thing>,

    /// A sub property of description. A short description of the item used to disambiguate from other, similar items. Information from other properties (in particular, name) may be necessary for the description to be useful for disambiguation.
    pub disambiguating_description: Option<Thing>,

    /// The identifier property represents any kind of identifier for any kind of <a class="localLink" href="/Thing">Thing</a>, such as ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing many of these, either as textual strings or as URL (URI) links. See <a href="/docs/datamodel.html#identifierBg">background notes</a> for more details.
    pub identifier: Option<Thing>,

    /// An image of the item. This can be a <a class="localLink" href="/URL">URL</a> or a fully described <a class="localLink" href="/ImageObject">ImageObject</a>.
    pub image: Option<Thing>,

    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being described. See <a href="/docs/datamodel.html#mainEntityBackground">background notes</a> for details.
    pub main_entity_of_page: Option<Thing>,

    /// The name of the item.
    pub name: Option<Thing>,

    /// Indicates a potential Action, which describes an idealized action in which this thing would play an 'object' role.
    pub potential_action: Option<Thing>,

    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL of the item's Wikipedia page, Wikidata entry, or official website.
    pub same_as: Option<Thing>,

    /// A CreativeWork or Event about this Thing.
    pub subject_of: Option<Thing>,

    /// URL of the item.
    pub url: Option<Thing>,

    /// The volume swept by all of the pistons inside the cylinders of an internal combustion engine in a single movement. <br/><br/>  Typical unit code(s): CMQ for cubic centimeter, LTR for liters, INQ for cubic inches * Note 1: You can link to information about how the given value has been determined using the <a class="localLink" href="/valueReference">valueReference</a> property. * Note 2: You can use <a class="localLink" href="/minValue">minValue</a> and <a class="localLink" href="/maxValue">maxValue</a> to indicate ranges.
    pub engine_displacement: Option<EngineSpecification>,

    /// The power of the vehicle's engine.     Typical unit code(s): KWT for kilowatt, BHP for brake horsepower, N12 for metric horsepower (PS, with 1 PS = 735,49875 W)<br/><br/>  <ul> <li>Note 1: There are many different ways of measuring an engine's power. For an overview, see  <a href="http://en.wikipedia.org/wiki/Horsepower#Engine_power_test_codes">http://en.wikipedia.org/wiki/Horsepower#Engine_power_test_codes</a>.</li> <li>Note 2: You can link to information about how the given value has been determined using the <a class="localLink" href="/valueReference">valueReference</a> property.</li> <li>Note 3: You can use <a class="localLink" href="/minValue">minValue</a> and <a class="localLink" href="/maxValue">maxValue</a> to indicate ranges.</li> </ul>
    pub engine_power: Option<EngineSpecification>,

    /// The type of engine or engines powering the vehicle.
    pub engine_type: Option<EngineSpecification>,

    /// The type of fuel suitable for the engine or engines of the vehicle. If the vehicle has only one engine, this property can be attached directly to the vehicle.
    pub fuel_type: Option<fuelType>,

    /// The torque (turning force) of the vehicle's engine.<br/><br/>  Typical unit code(s): NU for newton metre (N m), F17 for pound-force per foot, or F48 for pound-force per inch<br/><br/>  <ul> <li>Note 1: You can link to information about how the given value has been determined (e.g. reference RPM) using the <a class="localLink" href="/valueReference">valueReference</a> property.</li> <li>Note 2: You can use <a class="localLink" href="/minValue">minValue</a> and <a class="localLink" href="/maxValue">maxValue</a> to indicate ranges.</li> </ul>
    pub torque: Option<EngineSpecification>,
}

impl EngineSpecification {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
