use crate::prelude::*;

use super::text::Text;
use super::thing::Thing;
use super::url::URL;

/// The electronic systems used to play <a href="http://en.wikipedia.org/wiki/Category:Video_game_platforms">video games</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum gamePlatform {
    Text(Text),
    Thing(Thing),
    URL(URL),
}
