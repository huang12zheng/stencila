use crate::prelude::*;

use super::menu::Menu;
use super::menu_section::MenuSection;

/// A food or drink item contained in a menu or menu section.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum hasMenuItem {
    Menu(Menu),
    MenuSection(MenuSection),
}
