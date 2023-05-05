use crate::prelude::*;

use super::sports_event::SportsEvent;
use super::sports_organization::SportsOrganization;

/// A type of sport (e.g. Baseball).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum sport {
    SportsEvent(SportsEvent),
    SportsOrganization(SportsOrganization),
}
