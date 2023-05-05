use crate::prelude::*;

use super::audiobook::Audiobook;
use super::episode::Episode;
use super::event::Event;
use super::media_object::MediaObject;
use super::movie::Movie;
use super::music_recording::MusicRecording;
use super::music_release::MusicRelease;
use super::quantitative_value_distribution::QuantitativeValueDistribution;
use super::schedule::Schedule;

/// The duration of the item (movie, audio recording, event, etc.) in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum duration {
    Audiobook(Audiobook),
    Episode(Episode),
    Event(Event),
    MediaObject(MediaObject),
    Movie(Movie),
    MusicRecording(MusicRecording),
    MusicRelease(MusicRelease),
    QuantitativeValueDistribution(QuantitativeValueDistribution),
    Schedule(Schedule),
}
