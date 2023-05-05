use crate::prelude::*;

use super::answer::Answer;
use super::item_list::ItemList;


/// http://schema.org/suggestedAnswer
/// An answer (possibly one of several, possibly incorrect) to a Question, e.g. on a Question/Answer site.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SuggestedAnswerPropEnum {
    Answer(Answer),
    ItemList(ItemList),
}
