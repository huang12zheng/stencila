use crate::prelude::*;

use super::game::Game;
use super::video_game_series::VideoGameSeries;

/// Real or fictional location of the game (or part of game).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum gameLocation {
    Game(Game),
    VideoGameSeries(VideoGameSeries),
}
