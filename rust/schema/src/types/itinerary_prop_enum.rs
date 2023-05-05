use crate::prelude::*;

use super::item_list::ItemList;
use super::place::Place;


/// http://schema.org/itinerary
/// Destination(s) ( <a class="localLink" href="/Place">Place</a> ) that make up a trip. For a trip where destination order is important use <a class="localLink" href="/ItemList">ItemList</a> to specify that order (see examples).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ItineraryPropEnum {
    ItemList(ItemList),
    Place(Place),
}
