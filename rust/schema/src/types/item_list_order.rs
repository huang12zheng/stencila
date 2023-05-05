use crate::prelude::*;

use super::item_list_order_type::ItemListOrderType;
use super::text::Text;

/// Type of ordering (e.g. Ascending, Descending, Unordered).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum itemListOrder {
    ItemListOrderType(ItemListOrderType),
    Text(Text),
}
