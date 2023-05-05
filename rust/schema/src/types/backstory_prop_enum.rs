use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::text::Text;


/// http://schema.org/backstory
/// For an <a class="localLink" href="/Article">Article</a>, typically a <a class="localLink" href="/NewsArticle">NewsArticle</a>, the backstory property provides a textual summary giving a brief explanation of why and how an article was created. In a journalistic setting this could include information about reporting process, methods, interviews, data sources, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum BackstoryPropEnum {
    CreativeWork(CreativeWork),
    Text(Text),
}
