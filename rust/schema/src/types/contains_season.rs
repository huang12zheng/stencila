use crate::prelude::*;

use super::radio_series::RadioSeries;
use super::tv_series::TVSeries;
use super::video_game_series::VideoGameSeries;

/// A season that is part of the media series.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum containsSeason {
    RadioSeries(RadioSeries),
    TVSeries(TVSeries),
    VideoGameSeries(VideoGameSeries),
}
