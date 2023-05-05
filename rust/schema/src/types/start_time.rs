use crate::prelude::*;

use super::action::Action;
use super::food_establishment_reservation::FoodEstablishmentReservation;
use super::interaction_counter::InteractionCounter;
use super::media_object::MediaObject;
use super::schedule::Schedule;

/// The startTime of something. For a reserved event or service (e.g. FoodEstablishmentReservation), the time that it is expected to start. For actions that span a period of time, when the action was performed. E.g. John wrote a book from <em>January</em> to December. For media, including audio and video, it's the time offset of the start of a clip within a larger file.<br/><br/>  Note that Event uses startDate/endDate instead of startTime/endTime, even when describing dates with times. This situation may be clarified in future revisions.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum startTime {
    Action(Action),
    FoodEstablishmentReservation(FoodEstablishmentReservation),
    InteractionCounter(InteractionCounter),
    MediaObject(MediaObject),
    Schedule(Schedule),
}
