use crate::prelude::*;

use super::game::Game;
use super::video_game_series::VideoGameSeries;

/// Indicate how many people can play this game (minimum, maximum, or range).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum numberOfPlayers {
    Game(Game),
    VideoGameSeries(VideoGameSeries),
}
