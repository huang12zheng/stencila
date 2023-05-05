use crate::prelude::*;

use super::game::Game;
use super::video_game_series::VideoGameSeries;

/// An item is an object within the game world that can be collected by a player or, occasionally, a non-player character.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum gameItem {
    Game(Game),
    VideoGameSeries(VideoGameSeries),
}
