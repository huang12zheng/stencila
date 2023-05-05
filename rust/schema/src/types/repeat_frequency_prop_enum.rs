use crate::prelude::*;

use super::duration::Duration;
use super::text::Text;


/// http://schema.org/repeatFrequency
/// Defines the frequency at which <a class="localLink" href="/Event">Event</a>s will occur according to a schedule <a class="localLink" href="/Schedule">Schedule</a>. The intervals between       events should be defined as a <a class="localLink" href="/Duration">Duration</a> of time.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum RepeatFrequencyPropEnum {
    Duration(Duration),
    Text(Text),
}
