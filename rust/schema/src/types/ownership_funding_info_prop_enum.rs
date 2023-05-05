use crate::prelude::*;

use super::about_page::AboutPage;
use super::creative_work::CreativeWork;
use super::text::Text;
use super::url::URL;


/// http://schema.org/ownershipFundingInfo
/// For an <a class="localLink" href="/Organization">Organization</a> (often but not necessarily a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>), a description of organizational ownership structure; funding and grants. In a news/media setting, this is with particular reference to editorial independence.   Note that the <a class="localLink" href="/funder">funder</a> is also available and can be used to make basic funder information machine-readable.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum OwnershipFundingInfoPropEnum {
    AboutPage(AboutPage),
    CreativeWork(CreativeWork),
    Text(Text),
    URL(URL),
}
