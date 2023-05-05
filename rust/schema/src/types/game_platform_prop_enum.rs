use crate::prelude::*;

use super::text::Text;
use super::thing::Thing;
use super::url::URL;


/// http://schema.org/gamePlatform
/// The electronic systems used to play <a href="http://en.wikipedia.org/wiki/Category:Video_game_platforms">video games</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum GamePlatformPropEnum {
    Text(Text),
    Thing(Thing),
    URL(URL),
}
