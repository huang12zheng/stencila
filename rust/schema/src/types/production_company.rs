use crate::prelude::*;

use super::creative_work_season::CreativeWorkSeason;
use super::episode::Episode;
use super::media_object::MediaObject;
use super::movie::Movie;
use super::movie_series::MovieSeries;
use super::radio_series::RadioSeries;
use super::tv_series::TVSeries;
use super::video_game_series::VideoGameSeries;

/// The production company or studio responsible for the item, e.g. series, video game, episode etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum productionCompany {
    CreativeWorkSeason(CreativeWorkSeason),
    Episode(Episode),
    MediaObject(MediaObject),
    Movie(Movie),
    MovieSeries(MovieSeries),
    RadioSeries(RadioSeries),
    TVSeries(TVSeries),
    VideoGameSeries(VideoGameSeries),
}
