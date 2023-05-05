use crate::prelude::*;

use super::solve_math_action::SolveMathAction;
use super::text::Text;


/// http://schema.org/mathExpression
/// A mathematical expression (e.g. 'x^2-3x=0') that may be solved for a specific variable, simplified, or transformed. This can take many formats, e.g. LaTeX, Ascii-Math, or math as you would write with a keyboard.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum MathExpressionPropEnum {
    SolveMathAction(SolveMathAction),
    Text(Text),
}
