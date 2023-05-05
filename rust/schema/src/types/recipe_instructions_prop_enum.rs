use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::item_list::ItemList;
use super::text::Text;


/// http://schema.org/recipeInstructions
/// A step in making the recipe, in the form of a single item (document, video, etc.) or an ordered list with HowToStep and/or HowToSection items.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum RecipeInstructionsPropEnum {
    CreativeWork(CreativeWork),
    ItemList(ItemList),
    Text(Text),
}
