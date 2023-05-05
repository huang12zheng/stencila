use crate::prelude::*;

use super::exercise_action::ExerciseAction;
use super::exercise_plan::ExercisePlan;

/// Type(s) of exercise or activity, such as strength training, flexibility training, aerobics, cardiac rehabilitation, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum exerciseType {
    ExerciseAction(ExerciseAction),
    ExercisePlan(ExercisePlan),
}
