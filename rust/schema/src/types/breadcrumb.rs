use crate::prelude::*;

use super::breadcrumb_list::BreadcrumbList;
use super::text::Text;

/// A set of links that can help a user understand and navigate a website hierarchy.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum breadcrumb {
    BreadcrumbList(BreadcrumbList),
    Text(Text),
}
