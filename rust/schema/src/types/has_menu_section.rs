use crate::prelude::*;

use super::menu::Menu;
use super::menu_section::MenuSection;

/// A subgrouping of the menu (by dishes, course, serving time period, etc.).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum hasMenuSection {
    Menu(Menu),
    MenuSection(MenuSection),
}
