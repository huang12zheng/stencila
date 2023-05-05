use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;


/// http://schema.org/expires
/// Date the content expires and is no longer useful or available. For example a <a class="localLink" href="/VideoObject">VideoObject</a> or <a class="localLink" href="/NewsArticle">NewsArticle</a> whose availability or relevance is time-limited, or a <a class="localLink" href="/ClaimReview">ClaimReview</a> fact check whose publisher wants to indicate that it may no longer be relevant (or helpful to highlight) after some date.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ExpiresPropEnum {
    Date(Date),
    DateTime(DateTime),
}
