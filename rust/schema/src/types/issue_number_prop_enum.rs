use crate::prelude::*;

use super::integer::Integer;
use super::text::Text;


/// http://schema.org/issueNumber
/// Identifies the issue of publication; for example, "iii" or "2".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum IssueNumberPropEnum {
    Integer(Integer),
    Text(Text),
}
