use crate::prelude::*;

use super::video_game::VideoGame;
use super::video_game_series::VideoGameSeries;

/// Cheat codes to the game.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum cheatCode {
    VideoGame(VideoGame),
    VideoGameSeries(VideoGameSeries),
}
