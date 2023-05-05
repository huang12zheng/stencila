use crate::prelude::*;

use super::product::Product;
use super::review::Review;

/// Provides positive considerations regarding something, for example product highlights or (alongside <a class="localLink" href="/negativeNotes">negativeNotes</a>) pro/con lists for reviews.<br/><br/>  In the case of a <a class="localLink" href="/Review">Review</a>, the property describes the <a class="localLink" href="/itemReviewed">itemReviewed</a> from the perspective of the review; in the case of a <a class="localLink" href="/Product">Product</a>, the product itself is being described.<br/><br/>  The property values can be expressed either as unstructured text (repeated as necessary), or if ordered, as a list (in which case the most positive is at the beginning of the list).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum positiveNotes {
    Product(Product),
    Review(Review),
}
