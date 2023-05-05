use crate::prelude::*;

use super::clip::Clip;
use super::creative_work_season::CreativeWorkSeason;
use super::episode::Episode;
use super::event::Event;
use super::movie::Movie;
use super::movie_series::MovieSeries;
use super::radio_series::RadioSeries;
use super::tv_series::TVSeries;
use super::video_game::VideoGame;
use super::video_game_series::VideoGameSeries;
use super::video_object::VideoObject;

/// A director of e.g. TV, radio, movie, video gaming etc. content, or of an event. Directors can be associated with individual items or with a series, episode, clip.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum director {
    Clip(Clip),
    CreativeWorkSeason(CreativeWorkSeason),
    Episode(Episode),
    Event(Event),
    Movie(Movie),
    MovieSeries(MovieSeries),
    RadioSeries(RadioSeries),
    TVSeries(TVSeries),
    VideoGame(VideoGame),
    VideoGameSeries(VideoGameSeries),
    VideoObject(VideoObject),
}
