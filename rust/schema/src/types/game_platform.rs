use crate::prelude::*;

use super::video_game::VideoGame;
use super::video_game_series::VideoGameSeries;

/// The electronic systems used to play <a href="http://en.wikipedia.org/wiki/Category:Video_game_platforms">video games</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum gamePlatform {
    VideoGame(VideoGame),
    VideoGameSeries(VideoGameSeries),
}
