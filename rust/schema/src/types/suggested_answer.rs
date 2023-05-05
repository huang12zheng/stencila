use crate::prelude::*;

use super::answer::Answer;
use super::item_list::ItemList;

/// An answer (possibly one of several, possibly incorrect) to a Question, e.g. on a Question/Answer site.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum suggestedAnswer {
    Answer(Answer),
    ItemList(ItemList),
}
