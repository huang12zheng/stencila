use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::item_list::ItemList;
use super::text::Text;

/// A step in making the recipe, in the form of a single item (document, video, etc.) or an ordered list with HowToStep and/or HowToSection items.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum recipeInstructions {
    CreativeWork(CreativeWork),
    ItemList(ItemList),
    Text(Text),
}
