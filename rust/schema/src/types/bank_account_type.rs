use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// The type of a bank account.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum bankAccountType {
    Text(Text),
    URL(URL),
}
