// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::action_status_type::ActionStatusType;
use super::audience::Audience;
use super::diet::Diet;
use super::distance::Distance;
use super::event::Event;
use super::exercise_plan::ExercisePlan;
use super::person::Person;
use super::place::Place;
use super::sports_activity_location::SportsActivityLocation;
use super::sports_event::SportsEvent;
use super::sports_team::SportsTeam;
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
use super::start_time::startTime;
use super::subject_of::subjectOf;
use super::target::target;

/// * COMMENT: The act of participating in exertive activity for the purposes of improving health and fitness. * EXTEND FROM: https://schema.org/PlayAction
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ExerciseAction {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<ExerciseActionOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ExerciseActionOptions {
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

    /// An intended audience, i.e. a group for whom something was created.
    pub audience: Option<Audience>,

    /// Upcoming or past event associated with this place, organization, or action.
    pub event: Option<Event>,

    /// A sub property of instrument. The diet used in this action.
    pub diet: Option<Diet>,

    /// The distance travelled, e.g. exercising or travelling.
    pub distance: Option<Distance>,

    /// A sub property of location. The course where this action was taken.
    pub exercise_course: Option<Place>,

    /// A sub property of instrument. The exercise plan used on this action.
    pub exercise_plan: Option<ExercisePlan>,

    /// A sub property of instrument. The diet used in this action.
    pub exercise_related_diet: Option<Diet>,

    /// Type(s) of exercise or activity, such as strength training, flexibility training, aerobics, cardiac rehabilitation, etc.
    pub exercise_type: Option<Text>,

    /// A sub property of location. The original location of the object or the agent before the action.
    pub from_location: Option<Place>,

    /// A sub property of participant. The opponent on this action.
    pub opponent: Option<Person>,

    /// A sub property of location. The sports activity location where this action occurred.
    pub sports_activity_location: Option<SportsActivityLocation>,

    /// A sub property of location. The sports event where this action occurred.
    pub sports_event: Option<SportsEvent>,

    /// A sub property of participant. The sports team that participated on this action.
    pub sports_team: Option<SportsTeam>,

    /// A sub property of location. The final location of the object or the agent after the action.
    pub to_location: Option<Place>,
}

impl ExerciseAction {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
