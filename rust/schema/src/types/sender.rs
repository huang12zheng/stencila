use crate::prelude::*;

use super::message::Message;
use super::receive_action::ReceiveAction;

/// A sub property of participant. The participant who is at the sending end of the action.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum sender {
    Message(Message),
    ReceiveAction(ReceiveAction),
}
