use crate::prelude::*;

use super::exercise_action::ExerciseAction;
use super::insert_action::InsertAction;
use super::move_action::MoveAction;
use super::transfer_action::TransferAction;

/// A sub property of location. The final location of the object or the agent after the action.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum toLocation {
    ExerciseAction(ExerciseAction),
    InsertAction(InsertAction),
    MoveAction(MoveAction),
    TransferAction(TransferAction),
}
