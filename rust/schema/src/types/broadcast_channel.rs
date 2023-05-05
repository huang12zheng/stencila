// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::broadcast_frequency_prop_enum::BroadcastFrequencyPropEnum;
use super::broadcast_service::BroadcastService;
use super::cable_or_satellite_service::CableOrSatelliteService;
use super::genre_prop_enum::GenrePropEnum;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;

/// https://schema.org/BroadcastChannel
/// * COMMENT:
/// A unique instance of a BroadcastService on a CableOrSatelliteService lineup.
/// * EXTEND FROM:
/// https://schema.org/Intangible
/// * LOOK ALSO:
/// https://schema.org/RadioChannel, https://schema.org/TelevisionChannel
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct BroadcastChannel {
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

    /// The unique address by which the BroadcastService can be identified in a provider lineup. In US, this is typically a number.
    pub broadcast_channel_id_prop_enum: Option<Text>,

    /// The frequency used for over-the-air broadcasts. Numeric values or simple ranges, e.g. 87-99. In addition a shortcut idiom is supported for frequences of AM and FM radio channels, e.g. "87 FM".
    pub broadcast_frequency_prop_enum: Option<BroadcastFrequencyPropEnum>,

    /// The type of service required to have access to the channel (e.g. Standard or Premium).
    pub broadcast_service_tier_prop_enum: Option<Text>,

    /// Genre of the creative work, broadcast channel or group.
    pub genre_prop_enum: Option<GenrePropEnum>,

    /// The CableOrSatelliteService offering the channel.
    pub in_broadcast_lineup_prop_enum: Option<CableOrSatelliteService>,

    /// The BroadcastService offered on this channel.
    pub provides_broadcast_service_prop_enum: Option<BroadcastService>,
}

impl BroadcastChannel {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
