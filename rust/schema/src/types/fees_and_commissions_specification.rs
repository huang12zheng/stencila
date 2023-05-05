use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// Description of fees, commissions, and other terms applied either to a class of financial product, or by a financial service organization.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum feesAndCommissionsSpecification {
    Text(Text),
    URL(URL),
}
