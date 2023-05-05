use crate::prelude::*;

use super::duration::Duration;
use super::text::Text;

/// Defines the frequency at which <a class="localLink" href="/Event">Event</a>s will occur according to a schedule <a class="localLink" href="/Schedule">Schedule</a>. The intervals between       events should be defined as a <a class="localLink" href="/Duration">Duration</a> of time.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum repeatFrequency {
    Duration(Duration),
    Text(Text),
}
