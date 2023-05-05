use crate::prelude::*;

use super::question::Question;
use super::solve_math_action::SolveMathAction;

/// For questions that are part of learning resources (e.g. Quiz), eduQuestionType indicates the format of question being given. Example: "Multiple choice", "Open ended", "Flashcard".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum eduQuestionType {
    Question(Question),
    SolveMathAction(SolveMathAction),
}
