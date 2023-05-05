use crate::prelude::*;

use super::quantitative_value::QuantitativeValue;
use super::text::Text;


/// http://schema.org/molecularWeight
/// This is the molecular weight of the entity being described, not of the parent. Units should be included in the form '&lt;Number&gt; &lt;unit&gt;', for example '12 amu' or as '&lt;QuantitativeValue&gt;.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum MolecularWeightPropEnum {
    QuantitativeValue(QuantitativeValue),
    Text(Text),
}
