use crate::prelude::*;

use super::broadcast_channel::BroadcastChannel;
use super::broadcast_service::BroadcastService;

/// The frequency used for over-the-air broadcasts. Numeric values or simple ranges, e.g. 87-99. In addition a shortcut idiom is supported for frequences of AM and FM radio channels, e.g. "87 FM".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum broadcastFrequency {
    BroadcastChannel(BroadcastChannel),
    BroadcastService(BroadcastService),
}
