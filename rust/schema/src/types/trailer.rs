use crate::prelude::*;

use super::creative_work_season::CreativeWorkSeason;
use super::episode::Episode;
use super::movie::Movie;
use super::movie_series::MovieSeries;
use super::radio_series::RadioSeries;
use super::tv_series::TVSeries;
use super::video_game::VideoGame;
use super::video_game_series::VideoGameSeries;

/// The trailer of a movie or TV/radio series, season, episode, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum trailer {
    CreativeWorkSeason(CreativeWorkSeason),
    Episode(Episode),
    Movie(Movie),
    MovieSeries(MovieSeries),
    RadioSeries(RadioSeries),
    TVSeries(TVSeries),
    VideoGame(VideoGame),
    VideoGameSeries(VideoGameSeries),
}
