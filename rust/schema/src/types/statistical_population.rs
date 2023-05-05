// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::class::Class;
use super::integer::Integer;
use super::text::Text;
use super::url::URL;
use super::identifier::identifier;
use super::image::image;
use super::main_entity_of_page::mainEntityOfPage;
use super::subject_of::subjectOf;

/// * MOD OF: https://pending.schema.org * COMMENT: A StatisticalPopulation is a set of instances of a certain given type that satisfy some set of constraints. The property <a class="localLink" href="/populationType">populationType</a> is used to specify the type. Any property that can be used on instances of that type can appear on the statistical population. For example, a <a class="localLink" href="/StatisticalPopulation">StatisticalPopulation</a> representing all <a class="localLink" href="/Person">Person</a>s with a <a class="localLink" href="/homeLocation">homeLocation</a> of East Podunk California would be described by applying the appropriate <a class="localLink" href="/homeLocation">homeLocation</a> and <a class="localLink" href="/populationType">populationType</a> properties to a <a class="localLink" href="/StatisticalPopulation">StatisticalPopulation</a> item that stands for that set of people. The properties <a class="localLink" href="/numConstraints">numConstraints</a> and <a class="localLink" href="/constrainingProperty">constrainingProperty</a> are used to specify which of the populations properties are used to specify the population. Note that the sense of "population" used here is the general sense of a statistical population, and does not imply that the population consists of people. For example, a <a class="localLink" href="/populationType">populationType</a> of <a class="localLink" href="/Event">Event</a> or <a class="localLink" href="/NewsArticle">NewsArticle</a> could be used. See also <a class="localLink" href="/Observation">Observation</a>, and the <a href="/docs/data-and-datasets.html">data and datasets</a> overview for more details. * EXTEND FROM: https://schema.org/Intangible
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct StatisticalPopulation {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<StatisticalPopulationOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct StatisticalPopulationOptions {
    /// An additional type for the item, typically used for adding more specific types from external vocabularies in microdata syntax. This is a relationship between something and a class that the thing is in. In RDFa syntax, it is better to use the native RDFa syntax - the 'typeof' attribute - for multiple types. Schema.org tools may have only weaker understanding of extra types, in particular those defined externally.
    pub additional_type: Option<URL>,

    /// An alias for the item.
    pub alternate_name: Option<Text>,

    /// A description of the item.
    pub description: Option<Text>,

    /// A sub property of description. A short description of the item used to disambiguate from other, similar items. Information from other properties (in particular, name) may be necessary for the description to be useful for disambiguation.
    pub disambiguating_description: Option<Text>,

    /// The identifier property represents any kind of identifier for any kind of <a class="localLink" href="/Thing">Thing</a>, such as ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing many of these, either as textual strings or as URL (URI) links. See <a href="/docs/datamodel.html#identifierBg">background notes</a> for more details.
    pub identifier: Option<identifier>,

    /// An image of the item. This can be a <a class="localLink" href="/URL">URL</a> or a fully described <a class="localLink" href="/ImageObject">ImageObject</a>.
    pub image: Option<image>,

    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being described. See <a href="/docs/datamodel.html#mainEntityBackground">background notes</a> for details.
    pub main_entity_of_page: Option<mainEntityOfPage>,

    /// The name of the item.
    pub name: Option<Text>,

    /// Indicates a potential Action, which describes an idealized action in which this thing would play an 'object' role.
    pub potential_action: Option<Action>,

    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL of the item's Wikipedia page, Wikidata entry, or official website.
    pub same_as: Option<URL>,

    /// A CreativeWork or Event about this Thing.
    pub subject_of: Option<subjectOf>,

    /// URL of the item.
    pub url: Option<URL>,

    /// Indicates a property used as a constraint to define a <a class="localLink" href="/StatisticalPopulation">StatisticalPopulation</a> with respect to the set of entities   corresponding to an indicated type (via <a class="localLink" href="/populationType">populationType</a>).
    pub constraining_property: Option<Integer>,

    /// Indicates the number of constraints (not counting <a class="localLink" href="/populationType">populationType</a>) defined for a particular <a class="localLink" href="/StatisticalPopulation">StatisticalPopulation</a>. This helps applications understand if they have access to a sufficiently complete description of a <a class="localLink" href="/StatisticalPopulation">StatisticalPopulation</a>.
    pub num_constraints: Option<Integer>,

    /// Indicates the populationType common to all members of a <a class="localLink" href="/StatisticalPopulation">StatisticalPopulation</a>.
    pub population_type: Option<Class>,
}

impl StatisticalPopulation {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
