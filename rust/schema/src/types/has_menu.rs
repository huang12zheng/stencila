use crate::prelude::*;

use super::menu::Menu;
use super::text::Text;
use super::url::URL;

/// Either the actual menu as a structured representation, as text, or a URL of the menu.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum hasMenu {
    Menu(Menu),
    Text(Text),
    URL(URL),
}
