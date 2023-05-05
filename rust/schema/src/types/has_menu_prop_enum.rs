use crate::prelude::*;

use super::menu::Menu;
use super::text::Text;
use super::url::URL;


/// http://schema.org/hasMenu
/// Either the actual menu as a structured representation, as text, or a URL of the menu.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum HasMenuPropEnum {
    Menu(Menu),
    Text(Text),
    URL(URL),
}
