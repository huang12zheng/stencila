use crate::prelude::*;

use super::menu_item::MenuItem;
use super::menu_section::MenuSection;

/// Additional menu item(s) such as a side dish of salad or side order of fries that can be added to this menu item. Additionally it can be a menu section containing allowed add-on menu items for this menu item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum menuAddOn {
    MenuItem(MenuItem),
    MenuSection(MenuSection),
}
