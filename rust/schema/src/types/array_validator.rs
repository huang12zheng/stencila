// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::boolean::Boolean;
use super::integer::Integer;
use super::string::String;
use super::validator::Validator;

/// A validator specifying constraints on an array node.
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ArrayValidator {
    /// The type of this item
    pub r#type: MustBe!("ArrayValidator"),

    /// The identifier for this item
    pub id: Option<String>,

    /// Whether items can have the value `Node::Null`
    pub items_nullable: Boolean,

    /// Another validator node specifying the constraints on all items in the array.
    pub items_validator: Option<Box<Validator>>,

    /// An array node is valid if at least one of its items is valid against the `contains` schema.
    pub contains: Option<Box<Validator>>,

    /// An array node is valid if its size is greater than, or equal to, this value.
    pub min_items: Option<Integer>,

    /// An array node is valid if its size is less than, or equal to, this value.
    pub max_items: Option<Integer>,

    /// A flag to indicate that each value in the array should be unique.
    pub unique_items: Option<Boolean>,
}

impl ArrayValidator {
    pub fn new(items_nullable: Boolean) -> Self {
        Self {
            items_nullable,
            ..Default::default()
        }
    }
}
