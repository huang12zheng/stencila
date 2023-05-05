use crate::prelude::*;

use super::clip::Clip;
use super::creative_work_season::CreativeWorkSeason;
use super::episode::Episode;
use super::event::Event;
use super::movie::Movie;
use super::movie_series::MovieSeries;
use super::podcast_series::PodcastSeries;
use super::radio_series::RadioSeries;
use super::tv_series::TVSeries;
use super::video_game::VideoGame;
use super::video_game_series::VideoGameSeries;
use super::video_object::VideoObject;

/// An actor, e.g. in TV, radio, movie, video games etc., or in an event. Actors can be associated with individual items or with a series, episode, clip.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum actor {
    Clip(Clip),
    CreativeWorkSeason(CreativeWorkSeason),
    Episode(Episode),
    Event(Event),
    Movie(Movie),
    MovieSeries(MovieSeries),
    PodcastSeries(PodcastSeries),
    RadioSeries(RadioSeries),
    TVSeries(TVSeries),
    VideoGame(VideoGame),
    VideoGameSeries(VideoGameSeries),
    VideoObject(VideoObject),
}
