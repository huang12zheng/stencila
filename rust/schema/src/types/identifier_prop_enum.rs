use crate::prelude::*;

use super::property_value::PropertyValue;
use super::text::Text;
use super::url::URL;


/// http://schema.org/identifier
/// The identifier property represents any kind of identifier for any kind of <a class="localLink" href="/Thing">Thing</a>, such as ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing many of these, either as textual strings or as URL (URI) links. See <a href="/docs/datamodel.html#identifierBg">background notes</a> for more details.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum IdentifierPropEnum {
    PropertyValue(PropertyValue),
    Text(Text),
    URL(URL),
}
