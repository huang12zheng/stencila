use crate::prelude::*;

use super::item_list_order_type::ItemListOrderType;
use super::text::Text;


/// http://schema.org/itemListOrder
/// Type of ordering (e.g. Ascending, Descending, Unordered).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ItemListOrderPropEnum {
    ItemListOrderType(ItemListOrderType),
    Text(Text),
}
