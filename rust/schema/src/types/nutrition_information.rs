// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::energy::Energy;
use super::mass::Mass;
use super::text::Text;
use super::url::URL;
use super::identifier::identifier;
use super::image::image;
use super::main_entity_of_page::mainEntityOfPage;
use super::subject_of::subjectOf;

/// * COMMENT: Nutritional information about the recipe. * EXTEND FROM: https://schema.org/StructuredValue
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct NutritionInformation {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<NutritionInformationOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct NutritionInformationOptions {
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

    /// The number of calories.
    pub calories: Option<Energy>,

    /// The number of grams of carbohydrates.
    pub carbohydrate_content: Option<Mass>,

    /// The number of milligrams of cholesterol.
    pub cholesterol_content: Option<Mass>,

    /// The number of grams of fat.
    pub fat_content: Option<Mass>,

    /// The number of grams of fiber.
    pub fiber_content: Option<Mass>,

    /// The number of grams of protein.
    pub protein_content: Option<Mass>,

    /// The number of grams of saturated fat.
    pub saturated_fat_content: Option<Mass>,

    /// The serving size, in terms of the number of volume or mass.
    pub serving_size: Option<Text>,

    /// The number of milligrams of sodium.
    pub sodium_content: Option<Mass>,

    /// The number of grams of sugar.
    pub sugar_content: Option<Mass>,

    /// The number of grams of trans fat.
    pub trans_fat_content: Option<Mass>,

    /// The number of grams of unsaturated fat.
    pub unsaturated_fat_content: Option<Mass>,
}

impl NutritionInformation {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
