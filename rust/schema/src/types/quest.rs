use crate::prelude::*;

use super::game::Game;
use super::video_game_series::VideoGameSeries;

/// The task that a player-controlled character, or group of characters may complete in order to gain a reward.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum quest {
    Game(Game),
    VideoGameSeries(VideoGameSeries),
}
