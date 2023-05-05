use crate::prelude::*;

use super::number::Number;
use super::price_specification::PriceSpecification;
use super::text::Text;

/// The total price for the reservation or ticket, including applicable taxes, shipping, etc.<br/><br/>  Usage guidelines:<br/><br/>  <ul> <li>Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similar Unicode symbols.</li> <li>Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.</li> </ul>
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum totalPrice {
    Number(Number),
    PriceSpecification(PriceSpecification),
    Text(Text),
}
