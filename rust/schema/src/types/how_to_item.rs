// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::list_item::ListItem;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::position_prop_enum::PositionPropEnum;
use super::required_quantity_prop_enum::RequiredQuantityPropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::thing::Thing;
use super::url::URL;

/// https://schema.org/HowToItem
/// * COMMENT:
/// An item used as either a tool or supply when performing the instructions for how to achieve a result.
/// * EXTEND FROM:
/// https://schema.org/ListItem
/// * LOOK ALSO:
/// https://schema.org/HowToSupply, https://schema.org/HowToTool
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct HowToItem {
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

    /// An entity represented by an entry in a list or data feed (e.g. an 'artist' in a list of 'artists').
    pub item_prop_enum: Option<Thing>,

    /// A link to the ListItem that follows the current one.
    pub next_item_prop_enum: Option<ListItem>,

    /// The position of an item in a series or sequence of items.
    pub position_prop_enum: Option<PositionPropEnum>,

    /// A link to the ListItem that precedes the current one.
    pub previous_item_prop_enum: Option<ListItem>,

    /// The required quantity of the item(s).
    pub required_quantity_prop_enum: Option<RequiredQuantityPropEnum>,
}

impl HowToItem {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
