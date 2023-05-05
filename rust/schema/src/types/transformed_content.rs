// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::superseded_by_prop_enum::SupersededByPropEnum;
use super::text::Text;
use super::url::URL;

/// https://schema.org/TransformedContent
/// * MOD OF:
/// https://pending.schema.org
/// * COMMENT:
/// Content coded 'transformed content' in a <a class="localLink" href="/MediaReview">MediaReview</a>, considered in the context of how it was published or shared.<br/><br/>
/// 
/// For a <a class="localLink" href="/VideoObject">VideoObject</a> to be 'transformed content':  or all of the video has been manipulated to transform the footage itself. This category includes using tools like the Adobe Suite to change the speed of the video, add or remove visual elements or dub audio. Deepfakes are also a subset of transformation.<br/><br/>
/// 
/// For an <a class="localLink" href="/ImageObject">ImageObject</a> to be 'transformed content': Adding or deleting visual elements to give the image a different meaning with the intention to mislead.<br/><br/>
/// 
/// For an <a class="localLink" href="/ImageObject">ImageObject</a> with embedded text to be 'transformed content': Adding or deleting visual elements to give the image a different meaning with the intention to mislead.<br/><br/>
/// 
/// For an <a class="localLink" href="/AudioObject">AudioObject</a> to be 'transformed content': Part or all of the audio has been manipulated to alter the words or sounds, or the audio has been synthetically generated, such as to create a sound-alike voice.
/// * EXTEND FROM:
/// https://schema.org/MediaManipulationRatingEnumeration
/// * ENUMERATION FROM: https://schema.org/MediaManipulationRatingEnumeration
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct TransformedContent {
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

    /// Relates a term (i.e. a property, class or enumeration) to one that supersedes it.
    pub superseded_by_prop_enum: Option<SupersededByPropEnum>,
}

impl TransformedContent {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
