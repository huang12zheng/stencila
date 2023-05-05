use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::list_item::ListItem;

/// The position of an item in a series or sequence of items.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum position {
    CreativeWork(CreativeWork),
    ListItem(ListItem),
}
