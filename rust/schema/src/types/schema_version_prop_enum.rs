use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/schemaVersion
/// Indicates (by URL or string) a particular version of a schema used in some CreativeWork. This property was created primarily to     indicate the use of a specific schema.org release, e.g. <code>10.0</code> as a simple string, or more explicitly via URL, <code>http://schema.org/docs/releases.html#v10.0</code>. There may be situations in which other schemas might usefully be referenced this way, e.g. <code>http://dublincore.org/specifications/dublin-core/dces/1999-07-02/</code> but this has not been carefully explored in the community.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SchemaVersionPropEnum {
    Text(Text),
    URL(URL),
}
