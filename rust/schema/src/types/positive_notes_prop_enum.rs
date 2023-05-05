use crate::prelude::*;

use super::item_list::ItemList;
use super::list_item::ListItem;
use super::text::Text;
use super::web_content::WebContent;


/// http://schema.org/positiveNotes
/// Provides positive considerations regarding something, for example product highlights or (alongside <a class="localLink" href="/negativeNotes">negativeNotes</a>) pro/con lists for reviews.<br/><br/>  In the case of a <a class="localLink" href="/Review">Review</a>, the property describes the <a class="localLink" href="/itemReviewed">itemReviewed</a> from the perspective of the review; in the case of a <a class="localLink" href="/Product">Product</a>, the product itself is being described.<br/><br/>  The property values can be expressed either as unstructured text (repeated as necessary), or if ordered, as a list (in which case the most positive is at the beginning of the list).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum PositiveNotesPropEnum {
    ItemList(ItemList),
    ListItem(ListItem),
    Text(Text),
    WebContent(WebContent),
}
