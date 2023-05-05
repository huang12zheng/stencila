use crate::prelude::*;

use super::creative_work_season::CreativeWorkSeason;
use super::radio_series::RadioSeries;
use super::tv_series::TVSeries;
use super::video_game_series::VideoGameSeries;

/// The number of episodes in this season or series.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum numberOfEpisodes {
    CreativeWorkSeason(CreativeWorkSeason),
    RadioSeries(RadioSeries),
    TVSeries(TVSeries),
    VideoGameSeries(VideoGameSeries),
}
