use crate::prelude::*;

use super::radio_series::RadioSeries;
use super::tv_series::TVSeries;
use super::video_game_series::VideoGameSeries;

/// The number of seasons in this series.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum numberOfSeasons {
    RadioSeries(RadioSeries),
    TVSeries(TVSeries),
    VideoGameSeries(VideoGameSeries),
}
