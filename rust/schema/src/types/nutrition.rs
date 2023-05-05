use crate::prelude::*;

use super::menu_item::MenuItem;
use super::recipe::Recipe;

/// Nutrition information about the recipe or menu item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum nutrition {
    MenuItem(MenuItem),
    Recipe(Recipe),
}
