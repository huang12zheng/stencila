use common::itertools::Itertools;
use node_html::{elem, name, ToHtml};

use crate::Array;

impl ToHtml for Array {
    fn to_html(&self) -> String {
        elem(
            &name("Array"),
            &[],
            &[elem(
                "ol",
                &[],
                &self
                    .iter()
                    .map(|value| elem("li", &[], &[value.to_html()]))
                    .collect_vec(),
            )],
        )
    }
}
