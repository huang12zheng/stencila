use crate::prelude::*;

use super::menu_item::MenuItem;
use super::menu_section::MenuSection;


/// http://schema.org/menuAddOn
/// Additional menu item(s) such as a side dish of salad or side order of fries that can be added to this menu item. Additionally it can be a menu section containing allowed add-on menu items for this menu item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum MenuAddOnPropEnum {
    MenuItem(MenuItem),
    MenuSection(MenuSection),
}
