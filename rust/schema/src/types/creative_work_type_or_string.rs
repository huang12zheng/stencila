use crate::prelude::*;

use super::creative_work_type::CreativeWorkType;
use super::string::String;

/// [`CreativeWorkType`] or [`String`]
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum CreativeWorkTypeOrString {
    CreativeWorkType(CreativeWorkType),
    String(String),
}
