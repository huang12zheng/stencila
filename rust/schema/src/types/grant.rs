// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::block::Block;
use super::image_object_or_string::ImageObjectOrString;
use super::person_or_organization::PersonOrOrganization;
use super::property_value_or_string::PropertyValueOrString;
use super::string::String;
use super::thing::Thing;

/// A grant, typically financial or otherwise quantifiable, of resources.
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Grant {
    /// The type of this item
    pub r#type: MustBe!("Grant"),

    /// The identifier for this item
    pub id: Option<String>,

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<GrantOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct GrantOptions {
    /// Alternate names (aliases) for the item.
    pub alternate_names: Option<Vec<String>>,

    /// A description of the item.
    pub description: Option<Vec<Block>>,

    /// Any kind of identifier for any kind of Thing.
    pub identifiers: Option<Vec<PropertyValueOrString>>,

    /// Images of the item.
    pub images: Option<Vec<ImageObjectOrString>>,

    /// The name of the item.
    pub name: Option<String>,

    /// The URL of the item.
    pub url: Option<String>,

    /// Indicates an item funded or sponsored through a Grant.
    pub funded_items: Option<Vec<Thing>>,

    /// A person or organization that supports a thing through a pledge, promise, or financial contribution.
    pub sponsors: Option<Vec<PersonOrOrganization>>,
}

impl Grant {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
impl_into!(Grant, Thing);
impl_merge!(Grant, Thing);
