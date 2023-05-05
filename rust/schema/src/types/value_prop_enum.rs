use crate::prelude::*;

use super::boolean::Boolean;
use super::number::Number;
use super::structured_value::StructuredValue;
use super::text::Text;


/// http://schema.org/value
/// The value of the quantitative value or property value node.<br/><br/>  <ul> <li>For <a class="localLink" href="/QuantitativeValue">QuantitativeValue</a> and <a class="localLink" href="/MonetaryAmount">MonetaryAmount</a>, the recommended type for values is 'Number'.</li> <li>For <a class="localLink" href="/PropertyValue">PropertyValue</a>, it can be 'Text', 'Number', 'Boolean', or 'StructuredValue'.</li> <li>Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similar Unicode symbols.</li> <li>Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.</li> </ul>
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ValuePropEnum {
    Boolean(Boolean),
    Number(Number),
    StructuredValue(StructuredValue),
    Text(Text),
}
