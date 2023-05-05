// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::by_day_prop_enum::ByDayPropEnum;
use super::duration::Duration;
use super::end_date_prop_enum::EndDatePropEnum;
use super::end_time_prop_enum::EndTimePropEnum;
use super::except_date_prop_enum::ExceptDatePropEnum;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::integer::Integer;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::repeat_frequency_prop_enum::RepeatFrequencyPropEnum;
use super::start_date_prop_enum::StartDatePropEnum;
use super::start_time_prop_enum::StartTimePropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;

/// https://schema.org/Schedule
/// * MOD OF:
/// https://pending.schema.org
/// * COMMENT:
/// A schedule defines a repeating time period used to describe a regularly occurring <a class="localLink" href="/Event">Event</a>. At a minimum a schedule will specify <a class="localLink" href="/repeatFrequency">repeatFrequency</a> which describes the interval between occurrences of the event. Additional information can be provided to specify the schedule more precisely.
///       This includes identifying the day(s) of the week or month when the recurring event will take place, in addition to its start and end time. Schedules may also
///       have start and end dates to indicate when they are active, e.g. to define a limited calendar of events.
/// * EXTEND FROM:
/// https://schema.org/Intangible
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Schedule {
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

    /// Defines the day(s) of the week on which a recurring <a class="localLink" href="/Event">Event</a> takes place. May be specified using either <a class="localLink" href="/DayOfWeek">DayOfWeek</a>, or alternatively <a class="localLink" href="/Text">Text</a> conforming to iCal's syntax for byDay recurrence rules.
    pub by_day_prop_enum: Option<ByDayPropEnum>,

    /// Defines the month(s) of the year on which a recurring <a class="localLink" href="/Event">Event</a> takes place. Specified as an <a class="localLink" href="/Integer">Integer</a> between 1-12. January is 1.
    pub by_month_prop_enum: Option<Integer>,

    /// Defines the day(s) of the month on which a recurring <a class="localLink" href="/Event">Event</a> takes place. Specified as an <a class="localLink" href="/Integer">Integer</a> between 1-31.
    pub by_month_day_prop_enum: Option<Integer>,

    /// Defines the week(s) of the month on which a recurring Event takes place. Specified as an Integer between 1-5. For clarity, byMonthWeek is best used in conjunction with byDay to indicate concepts like the first and third Mondays of a month.
    pub by_month_week_prop_enum: Option<Integer>,

    /// The duration of the item (movie, audio recording, event, etc.) in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a>.
    pub duration_prop_enum: Option<Duration>,

    /// The end date and time of the item (in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a>).
    pub end_date_prop_enum: Option<EndDatePropEnum>,

    /// The endTime of something. For a reserved event or service (e.g. FoodEstablishmentReservation), the time that it is expected to end. For actions that span a period of time, when the action was performed. E.g. John wrote a book from January to <em>December</em>. For media, including audio and video, it's the time offset of the end of a clip within a larger file.<br/><br/>  Note that Event uses startDate/endDate instead of startTime/endTime, even when describing dates with times. This situation may be clarified in future revisions.
    pub end_time_prop_enum: Option<EndTimePropEnum>,

    /// Defines a <a class="localLink" href="/Date">Date</a> or <a class="localLink" href="/DateTime">DateTime</a> during which a scheduled <a class="localLink" href="/Event">Event</a> will not take place. The property allows exceptions to       a <a class="localLink" href="/Schedule">Schedule</a> to be specified. If an exception is specified as a <a class="localLink" href="/DateTime">DateTime</a> then only the event that would have started at that specific date and time       should be excluded from the schedule. If an exception is specified as a <a class="localLink" href="/Date">Date</a> then any event that is scheduled for that 24 hour period should be       excluded from the schedule. This allows a whole day to be excluded from the schedule without having to itemise every scheduled event.
    pub except_date_prop_enum: Option<ExceptDatePropEnum>,

    /// Defines the number of times a recurring <a class="localLink" href="/Event">Event</a> will take place.
    pub repeat_count_prop_enum: Option<Integer>,

    /// Defines the frequency at which <a class="localLink" href="/Event">Event</a>s will occur according to a schedule <a class="localLink" href="/Schedule">Schedule</a>. The intervals between       events should be defined as a <a class="localLink" href="/Duration">Duration</a> of time.
    pub repeat_frequency_prop_enum: Option<RepeatFrequencyPropEnum>,

    /// Indicates the timezone for which the time(s) indicated in the <a class="localLink" href="/Schedule">Schedule</a> are given. The value provided should be among those listed in the IANA Time Zone Database.
    pub schedule_timezone_prop_enum: Option<Text>,

    /// The start date and time of the item (in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a>).
    pub start_date_prop_enum: Option<StartDatePropEnum>,

    /// The startTime of something. For a reserved event or service (e.g. FoodEstablishmentReservation), the time that it is expected to start. For actions that span a period of time, when the action was performed. E.g. John wrote a book from <em>January</em> to December. For media, including audio and video, it's the time offset of the start of a clip within a larger file.<br/><br/>  Note that Event uses startDate/endDate instead of startTime/endTime, even when describing dates with times. This situation may be clarified in future revisions.
    pub start_time_prop_enum: Option<StartTimePropEnum>,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
