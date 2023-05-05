// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::action_status_type::ActionStatusType;
use super::delivery_method::DeliveryMethod;
use super::place::Place;
use super::text::Text;
use super::thing::Thing;
use super::url::URL;
use super::agent::agent;
use super::end_time::endTime;
use super::identifier::identifier;
use super::image::image;
use super::location::location;
use super::main_entity_of_page::mainEntityOfPage;
use super::participant::participant;
use super::provider::provider;
use super::sender::sender;
use super::start_time::startTime;
use super::subject_of::subjectOf;
use super::target::target;

/// * COMMENT: The act of physically/electronically taking delivery of an object that has been transferred from an origin to a destination. Reciprocal of SendAction.<br/><br/>  Related actions:<br/><br/>  <ul> <li><a class="localLink" href="/SendAction">SendAction</a>: The reciprocal of ReceiveAction.</li> <li><a class="localLink" href="/TakeAction">TakeAction</a>: Unlike TakeAction, ReceiveAction does not imply that the ownership has been transferred (e.g. I can receive a package, but it does not mean the package is now mine).</li> </ul> * EXTEND FROM: https://schema.org/TransferAction
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ReceiveAction {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<ReceiveActionOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ReceiveActionOptions {
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

    /// Indicates the current disposition of the Action.
    pub action_status: Option<ActionStatusType>,

    /// The direct performer or driver of the action (animate or inanimate). E.g. <em>John</em> wrote a book.
    pub agent: Option<agent>,

    /// The endTime of something. For a reserved event or service (e.g. FoodEstablishmentReservation), the time that it is expected to end. For actions that span a period of time, when the action was performed. E.g. John wrote a book from January to <em>December</em>. For media, including audio and video, it's the time offset of the end of a clip within a larger file.<br/><br/>  Note that Event uses startDate/endDate instead of startTime/endTime, even when describing dates with times. This situation may be clarified in future revisions.
    pub end_time: Option<endTime>,

    /// For failed actions, more information on the cause of the failure.
    pub error: Option<Thing>,

    /// The object that helped the agent perform the action. E.g. John wrote a book with <em>a pen</em>.
    pub instrument: Option<Thing>,

    /// The location of, for example, where an event is happening, where an organization is located, or where an action takes place.
    pub location: Option<location>,

    /// The object upon which the action is carried out, whose state is kept intact or changed. Also known as the semantic roles patient, affected or undergoer (which change their state) or theme (which doesn't). E.g. John read <em>a book</em>.
    pub object: Option<Thing>,

    /// Other co-agents that participated in the action indirectly. E.g. John wrote a book with <em>Steve</em>.
    pub participant: Option<participant>,

    /// The service provider, service operator, or service performer; the goods producer. Another party (a seller) may offer those services or goods on behalf of the provider. A provider may also serve as the seller.
    pub provider: Option<provider>,

    /// The result produced in the action. E.g. John wrote <em>a book</em>.
    pub result: Option<Thing>,

    /// The startTime of something. For a reserved event or service (e.g. FoodEstablishmentReservation), the time that it is expected to start. For actions that span a period of time, when the action was performed. E.g. John wrote a book from <em>January</em> to December. For media, including audio and video, it's the time offset of the start of a clip within a larger file.<br/><br/>  Note that Event uses startDate/endDate instead of startTime/endTime, even when describing dates with times. This situation may be clarified in future revisions.
    pub start_time: Option<startTime>,

    /// Indicates a target EntryPoint, or url, for an Action.
    pub target: Option<target>,

    /// A sub property of location. The original location of the object or the agent before the action.
    pub from_location: Option<Place>,

    /// A sub property of location. The final location of the object or the agent after the action.
    pub to_location: Option<Place>,

    /// A sub property of instrument. The method of delivery.
    pub delivery_method: Option<DeliveryMethod>,

    /// A sub property of participant. The participant who is at the sending end of the action.
    pub sender: Option<sender>,
}

impl ReceiveAction {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
