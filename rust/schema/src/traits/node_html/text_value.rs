use node_html::{text, ToHtml};

use crate::TextValue;

impl ToHtml for TextValue {
    fn to_html(&self) -> String {
        text(&self.0)
    }
}
