use crate::prelude::*;

use super::authorize_action::AuthorizeAction;
use super::communicate_action::CommunicateAction;
use super::donate_action::DonateAction;
use super::give_action::GiveAction;
use super::message::Message;
use super::pay_action::PayAction;
use super::return_action::ReturnAction;
use super::send_action::SendAction;
use super::tip_action::TipAction;

/// A sub property of participant. The participant who is at the receiving end of the action.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum recipient {
    AuthorizeAction(AuthorizeAction),
    CommunicateAction(CommunicateAction),
    DonateAction(DonateAction),
    GiveAction(GiveAction),
    Message(Message),
    PayAction(PayAction),
    ReturnAction(ReturnAction),
    SendAction(SendAction),
    TipAction(TipAction),
}
