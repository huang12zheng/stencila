use crate::prelude::*;

use super::comment::Comment;
use super::web_content::WebContent;


/// http://schema.org/answerExplanation
/// A step-by-step or full explanation about Answer. Can outline how this Answer was achieved or contain more broad clarification or statement about it.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AnswerExplanationPropEnum {
    Comment(Comment),
    WebContent(WebContent),
}
