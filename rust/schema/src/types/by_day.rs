use crate::prelude::*;

use super::day_of_week::DayOfWeek;
use super::text::Text;

/// Defines the day(s) of the week on which a recurring <a class="localLink" href="/Event">Event</a> takes place. May be specified using either <a class="localLink" href="/DayOfWeek">DayOfWeek</a>, or alternatively <a class="localLink" href="/Text">Text</a> conforming to iCal's syntax for byDay recurrence rules.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum byDay {
    DayOfWeek(DayOfWeek),
    Text(Text),
}
