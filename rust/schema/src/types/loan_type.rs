use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// The type of a loan or credit.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum loanType {
    Text(Text),
    URL(URL),
}
