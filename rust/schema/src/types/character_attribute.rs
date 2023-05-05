use crate::prelude::*;

use super::game::Game;
use super::video_game_series::VideoGameSeries;

/// A piece of data that represents a particular aspect of a fictional character (skill, power, character points, advantage, disadvantage).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum characterAttribute {
    Game(Game),
    VideoGameSeries(VideoGameSeries),
}
