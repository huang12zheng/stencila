//! Generated file, do not edit

use crate::prelude::*;

use super::string::String;

/// A thematic break, such as a scene change in a story, a transition to another topic, or a new document.
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ThematicBreak {
    /// The type of this item
    pub r#type: MustBe!("ThematicBreak"),

    /// The identifier for this item
    pub id: Option<String>,
}

impl ThematicBreak {
    #[rustfmt::skip]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}