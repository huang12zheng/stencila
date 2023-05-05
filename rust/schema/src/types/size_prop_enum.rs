use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::quantitative_value::QuantitativeValue;
use super::size_specification::SizeSpecification;
use super::text::Text;


/// http://schema.org/size
/// A standardized size of a product or creative work, specified either through a simple textual string (for example 'XL', '32Wx34L'), a  QuantitativeValue with a unitCode, or a comprehensive and structured <a class="localLink" href="/SizeSpecification">SizeSpecification</a>; in other cases, the <a class="localLink" href="/width">width</a>, <a class="localLink" href="/height">height</a>, <a class="localLink" href="/depth">depth</a> and <a class="localLink" href="/weight">weight</a> properties may be more applicable.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SizePropEnum {
    DefinedTerm(DefinedTerm),
    QuantitativeValue(QuantitativeValue),
    SizeSpecification(SizeSpecification),
    Text(Text),
}
