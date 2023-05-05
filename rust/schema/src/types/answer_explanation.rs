use crate::prelude::*;

use super::comment::Comment;
use super::web_content::WebContent;

/// A step-by-step or full explanation about Answer. Can outline how this Answer was achieved or contain more broad clarification or statement about it.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum answerExplanation {
    Comment(Comment),
    WebContent(WebContent),
}
