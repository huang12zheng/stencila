use crate::prelude::*;

use super::answer::Answer;
use super::item_list::ItemList;


/// http://schema.org/acceptedAnswer
/// The answer(s) that has been accepted as best, typically on a Question/Answer site. Sites vary in their selection mechanisms, e.g. drawing on community opinion and/or the view of the Question author.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AcceptedAnswerPropEnum {
    Answer(Answer),
    ItemList(ItemList),
}
