use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/editEIDR
/// An <a href="https://eidr.org/">EIDR</a> (Entertainment Identifier Registry) <a class="localLink" href="/identifier">identifier</a> representing a specific edit / edition for a work of film or television.<br/><br/>  For example, the motion picture known as "Ghostbusters" whose <a class="localLink" href="/titleEIDR">titleEIDR</a> is "10.5240/7EC7-228A-510A-053E-CBB8-J" has several edits, e.g. "10.5240/1F2A-E1C5-680A-14C6-E76B-I" and "10.5240/8A35-3BEE-6497-5D12-9E4F-3".<br/><br/>  Since schema.org types like <a class="localLink" href="/Movie">Movie</a> and <a class="localLink" href="/TVEpisode">TVEpisode</a> can be used for both works and their multiple expressions, it is possible to use <a class="localLink" href="/titleEIDR">titleEIDR</a> alone (for a general description), or alongside <a class="localLink" href="/editEIDR">editEIDR</a> for a more edit-specific description.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum EditEIDRPropEnum {
    Text(Text),
    URL(URL),
}
