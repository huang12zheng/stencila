use crate::prelude::*;

use super::solve_math_action::SolveMathAction;
use super::text::Text;

/// A mathematical expression (e.g. 'x^2-3x=0') that may be solved for a specific variable, simplified, or transformed. This can take many formats, e.g. LaTeX, Ascii-Math, or math as you would write with a keyboard.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum mathExpression {
    SolveMathAction(SolveMathAction),
    Text(Text),
}
