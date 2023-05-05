// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::aggregate_rating::AggregateRating;
use super::audience::Audience;
use super::boolean::Boolean;
use super::creative_work::CreativeWork;
use super::date::Date;
use super::duration::Duration;
use super::event::Event;
use super::event_attendance_mode_enumeration::EventAttendanceModeEnumeration;
use super::event_status_type::EventStatusType;
use super::grant::Grant;
use super::integer::Integer;
use super::person::Person;
use super::review::Review;
use super::schedule::Schedule;
use super::text::Text;
use super::thing::Thing;
use super::url::URL;
use super::attendee::attendee;
use super::composer::composer;
use super::contributor::contributor;
use super::door_time::doorTime;
use super::end_date::endDate;
use super::funder::funder;
use super::identifier::identifier;
use super::image::image;
use super::in_language::inLanguage;
use super::keywords::keywords;
use super::location::location;
use super::main_entity_of_page::mainEntityOfPage;
use super::offers::offers;
use super::organizer::organizer;
use super::performer::performer;
use super::sponsor::sponsor;
use super::start_date::startDate;
use super::subject_of::subjectOf;
use super::translator::translator;

/// * COMMENT: Event type: Theater performance. * EXTEND FROM: https://schema.org/Event
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct TheaterEvent {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<TheaterEventOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct TheaterEventOptions {
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

    /// The subject matter of the content.
    pub about: Option<Thing>,

    /// An actor, e.g. in TV, radio, movie, video games etc., or in an event. Actors can be associated with individual items or with a series, episode, clip.
    pub actor: Option<Person>,

    /// The overall rating, based on a collection of reviews or ratings, of the item.
    pub aggregate_rating: Option<AggregateRating>,

    /// A person or organization attending the event.
    pub attendee: Option<attendee>,

    /// An intended audience, i.e. a group for whom something was created.
    pub audience: Option<Audience>,

    /// The person or organization who wrote a composition, or who is the composer of a work performed at some event.
    pub composer: Option<composer>,

    /// A secondary contributor to the CreativeWork or Event.
    pub contributor: Option<contributor>,

    /// A director of e.g. TV, radio, movie, video gaming etc. content, or of an event. Directors can be associated with individual items or with a series, episode, clip.
    pub director: Option<Person>,

    /// The time admission will commence.
    pub door_time: Option<doorTime>,

    /// The duration of the item (movie, audio recording, event, etc.) in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a>.
    pub duration: Option<Duration>,

    /// The end date and time of the item (in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a>).
    pub end_date: Option<endDate>,

    /// The eventAttendanceMode of an event indicates whether it occurs online, offline, or a mix.
    pub event_attendance_mode: Option<EventAttendanceModeEnumeration>,

    /// Associates an <a class="localLink" href="/Event">Event</a> with a <a class="localLink" href="/Schedule">Schedule</a>. There are circumstances where it is preferable to share a schedule for a series of       repeating events rather than data on the individual events themselves. For example, a website or application might prefer to publish a schedule for a weekly       gym class rather than provide data on every event. A schedule could be processed by applications to add forthcoming events to a calendar. An <a class="localLink" href="/Event">Event</a> that       is associated with a <a class="localLink" href="/Schedule">Schedule</a> using this property should not have <a class="localLink" href="/startDate">startDate</a> or <a class="localLink" href="/endDate">endDate</a> properties. These are instead defined within the associated       <a class="localLink" href="/Schedule">Schedule</a>, this avoids any ambiguity for clients using the data. The property might have repeated values to specify different schedules, e.g. for different months       or seasons.
    pub event_schedule: Option<Schedule>,

    /// An eventStatus of an event represents its status; particularly useful when an event is cancelled or rescheduled.
    pub event_status: Option<EventStatusType>,

    /// A person or organization that supports (sponsors) something through some kind of financial contribution.
    pub funder: Option<funder>,

    /// A <a class="localLink" href="/Grant">Grant</a> that directly or indirectly provide funding or sponsorship for this item. See also <a class="localLink" href="/ownershipFundingInfo">ownershipFundingInfo</a>.
    pub funding: Option<Grant>,

    /// The language of the content or performance or used in an action. Please use one of the language codes from the <a href="http://tools.ietf.org/html/bcp47">IETF BCP 47 standard</a>. See also <a class="localLink" href="/availableLanguage">availableLanguage</a>.
    pub in_language: Option<inLanguage>,

    /// A flag to signal that the item, event, or place is accessible for free.
    pub is_accessible_for_free: Option<Boolean>,

    /// Keywords or tags used to describe some item. Multiple textual entries in a keywords list are typically delimited by commas, or by repeating the property.
    pub keywords: Option<keywords>,

    /// The location of, for example, where an event is happening, where an organization is located, or where an action takes place.
    pub location: Option<location>,

    /// The total number of individuals that may attend an event or venue.
    pub maximum_attendee_capacity: Option<Integer>,

    /// The maximum physical attendee capacity of an <a class="localLink" href="/Event">Event</a> whose <a class="localLink" href="/eventAttendanceMode">eventAttendanceMode</a> is <a class="localLink" href="/OfflineEventAttendanceMode">OfflineEventAttendanceMode</a> (or the offline aspects, in the case of a <a class="localLink" href="/MixedEventAttendanceMode">MixedEventAttendanceMode</a>).
    pub maximum_physical_attendee_capacity: Option<Integer>,

    /// The maximum physical attendee capacity of an <a class="localLink" href="/Event">Event</a> whose <a class="localLink" href="/eventAttendanceMode">eventAttendanceMode</a> is <a class="localLink" href="/OnlineEventAttendanceMode">OnlineEventAttendanceMode</a> (or the online aspects, in the case of a <a class="localLink" href="/MixedEventAttendanceMode">MixedEventAttendanceMode</a>).
    pub maximum_virtual_attendee_capacity: Option<Integer>,

    /// An offer to provide this item&#x2014;for example, an offer to sell a product, rent the DVD of a movie, perform a service, or give away tickets to an event. Use <a class="localLink" href="/businessFunction">businessFunction</a> to indicate the kind of transaction offered, i.e. sell, lease, etc. This property can also be used to describe a <a class="localLink" href="/Demand">Demand</a>. While this property is listed as expected on a number of common types, it can be used in others. In that case, using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
    pub offers: Option<offers>,

    /// An organizer of an Event.
    pub organizer: Option<organizer>,

    /// A performer at the event&#x2014;for example, a presenter, musician, musical group or actor.
    pub performer: Option<performer>,

    /// Used in conjunction with eventStatus for rescheduled or cancelled events. This property contains the previously scheduled start date. For rescheduled events, the startDate property should be used for the newly scheduled start date. In the (rare) case of an event that has been postponed and rescheduled multiple times, this field may be repeated.
    pub previous_start_date: Option<Date>,

    /// The CreativeWork that captured all or part of this Event.
    pub recorded_in: Option<CreativeWork>,

    /// The number of attendee places for an event that remain unallocated.
    pub remaining_attendee_capacity: Option<Integer>,

    /// A review of the item.
    pub review: Option<Review>,

    /// A person or organization that supports a thing through a pledge, promise, or financial contribution. E.g. a sponsor of a Medical Study or a corporate sponsor of an event.
    pub sponsor: Option<sponsor>,

    /// The start date and time of the item (in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a>).
    pub start_date: Option<startDate>,

    /// An Event that is part of this event. For example, a conference event includes many presentations, each of which is a subEvent of the conference.
    pub sub_event: Option<Event>,

    /// An event that this event is a part of. For example, a collection of individual music performances might each have a music festival as their superEvent.
    pub super_event: Option<Event>,

    /// Organization or person who adapts a creative work to different languages, regional differences and technical requirements of a target market, or that translates during some event.
    pub translator: Option<translator>,

    /// The typical expected age range, e.g. '7-9', '11-'.
    pub typical_age_range: Option<Text>,

    /// A work featured in some event, e.g. exhibited in an ExhibitionEvent.        Specific subproperties are available for workPerformed (e.g. a play), or a workPresented (a Movie at a ScreeningEvent).
    pub work_featured: Option<CreativeWork>,

    /// A work performed in some event, for example a play performed in a TheaterEvent.
    pub work_performed: Option<CreativeWork>,
}

impl TheaterEvent {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}