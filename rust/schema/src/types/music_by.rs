use crate::prelude::*;

use super::clip::Clip;
use super::episode::Episode;
use super::movie::Movie;
use super::movie_series::MovieSeries;
use super::radio_series::RadioSeries;
use super::tv_series::TVSeries;
use super::video_game::VideoGame;
use super::video_game_series::VideoGameSeries;
use super::video_object::VideoObject;

/// The composer of the soundtrack.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum musicBy {
    Clip(Clip),
    Episode(Episode),
    Movie(Movie),
    MovieSeries(MovieSeries),
    RadioSeries(RadioSeries),
    TVSeries(TVSeries),
    VideoGame(VideoGame),
    VideoGameSeries(VideoGameSeries),
    VideoObject(VideoObject),
}
