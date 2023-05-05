use crate::prelude::*;

use super::answer::Answer;
use super::item_list::ItemList;

/// The answer(s) that has been accepted as best, typically on a Question/Answer site. Sites vary in their selection mechanisms, e.g. drawing on community opinion and/or the view of the Question author.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum acceptedAnswer {
    Answer(Answer),
    ItemList(ItemList),
}
