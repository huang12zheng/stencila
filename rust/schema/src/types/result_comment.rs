use crate::prelude::*;

use super::comment_action::CommentAction;
use super::reply_action::ReplyAction;

/// A sub property of result. The Comment created or sent as a result of this action.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum resultComment {
    CommentAction(CommentAction),
    ReplyAction(ReplyAction),
}
