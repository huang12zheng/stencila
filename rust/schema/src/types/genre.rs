use crate::prelude::*;

use super::broadcast_channel::BroadcastChannel;
use super::creative_work::CreativeWork;
use super::music_group::MusicGroup;

/// Genre of the creative work, broadcast channel or group.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum genre {
    BroadcastChannel(BroadcastChannel),
    CreativeWork(CreativeWork),
    MusicGroup(MusicGroup),
}
