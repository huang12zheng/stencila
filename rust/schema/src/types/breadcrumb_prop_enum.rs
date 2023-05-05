use crate::prelude::*;

use super::breadcrumb_list::BreadcrumbList;
use super::text::Text;


/// http://schema.org/breadcrumb
/// A set of links that can help a user understand and navigate a website hierarchy.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum BreadcrumbPropEnum {
    BreadcrumbList(BreadcrumbList),
    Text(Text),
}
