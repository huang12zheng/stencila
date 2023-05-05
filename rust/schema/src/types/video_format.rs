use crate::prelude::*;

use super::broadcast_event::BroadcastEvent;
use super::broadcast_service::BroadcastService;
use super::screening_event::ScreeningEvent;

/// The type of screening or video broadcast used (e.g. IMAX, 3D, SD, HD, etc.).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum videoFormat {
    BroadcastEvent(BroadcastEvent),
    BroadcastService(BroadcastService),
    ScreeningEvent(ScreeningEvent),
}
