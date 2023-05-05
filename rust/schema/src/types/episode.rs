use crate::prelude::*;

use super::creative_work_season::CreativeWorkSeason;
use super::radio_series::RadioSeries;
use super::tv_series::TVSeries;
use super::video_game_series::VideoGameSeries;

/// An episode of a TV, radio or game media within a series or season.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum episode {
    CreativeWorkSeason(CreativeWorkSeason),
    RadioSeries(RadioSeries),
    TVSeries(TVSeries),
    VideoGameSeries(VideoGameSeries),
}
