use crate::prelude::*;

use super::exercise_action::ExerciseAction;
use super::move_action::MoveAction;
use super::transfer_action::TransferAction;

/// A sub property of location. The original location of the object or the agent before the action.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum fromLocation {
    ExerciseAction(ExerciseAction),
    MoveAction(MoveAction),
    TransferAction(TransferAction),
}
