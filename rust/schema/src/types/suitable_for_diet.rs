use crate::prelude::*;

use super::menu_item::MenuItem;
use super::recipe::Recipe;

/// Indicates a dietary restriction or guideline for which this recipe or menu item is suitable, e.g. diabetic, halal etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum suitableForDiet {
    MenuItem(MenuItem),
    Recipe(Recipe),
}
