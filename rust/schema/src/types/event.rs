use crate::prelude::*;

use super::inform_action::InformAction;
use super::invite_action::InviteAction;
use super::join_action::JoinAction;
use super::leave_action::LeaveAction;
use super::organization::Organization;
use super::place::Place;
use super::play_action::PlayAction;

/// Upcoming or past event associated with this place, organization, or action.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum event {
    InformAction(InformAction),
    InviteAction(InviteAction),
    JoinAction(JoinAction),
    LeaveAction(LeaveAction),
    Organization(Organization),
    Place(Place),
    PlayAction(PlayAction),
}
