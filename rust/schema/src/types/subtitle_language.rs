use crate::prelude::*;

use super::broadcast_event::BroadcastEvent;
use super::movie::Movie;
use super::screening_event::ScreeningEvent;
use super::tv_episode::TVEpisode;

/// Languages in which subtitles/captions are available, in <a href="http://tools.ietf.org/html/bcp47">IETF BCP 47 standard format</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum subtitleLanguage {
    BroadcastEvent(BroadcastEvent),
    Movie(Movie),
    ScreeningEvent(ScreeningEvent),
    TVEpisode(TVEpisode),
}
