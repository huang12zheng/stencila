use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/feesAndCommissionsSpecification
/// Description of fees, commissions, and other terms applied either to a class of financial product, or by a financial service organization.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum FeesAndCommissionsSpecificationPropEnum {
    Text(Text),
    URL(URL),
}
