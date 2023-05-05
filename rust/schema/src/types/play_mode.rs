use crate::prelude::*;

use super::video_game::VideoGame;
use super::video_game_series::VideoGameSeries;

/// Indicates whether this game is multi-player, co-op or single-player.  The game can be marked as multi-player, co-op and single-player at the same time.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum playMode {
    VideoGame(VideoGame),
    VideoGameSeries(VideoGameSeries),
}
