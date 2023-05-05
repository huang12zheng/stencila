// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::text::Text;
use super::url::URL;
use super::funded_item::fundedItem;
use super::funder::funder;
use super::identifier::identifier;
use super::image::image;
use super::main_entity_of_page::mainEntityOfPage;
use super::sponsor::sponsor;
use super::subject_of::subjectOf;

/// * MOD OF: https://pending.schema.org * COMMENT: A grant, typically financial or otherwise quantifiable, of resources. Typically a <a class="localLink" href="/funder">funder</a> sponsors some <a class="localLink" href="/MonetaryAmount">MonetaryAmount</a> to an <a class="localLink" href="/Organization">Organization</a> or <a class="localLink" href="/Person">Person</a>,     sometimes not necessarily via a dedicated or long-lived <a class="localLink" href="/Project">Project</a>, resulting in one or more outputs, or <a class="localLink" href="/fundedItem">fundedItem</a>s. For financial sponsorship, indicate the <a class="localLink" href="/funder">funder</a> of a <a class="localLink" href="/MonetaryGrant">MonetaryGrant</a>. For non-financial support, indicate <a class="localLink" href="/sponsor">sponsor</a> of <a class="localLink" href="/Grant">Grant</a>s of resources (e.g. office space).<br/><br/>  Grants support  activities directed towards some agreed collective goals, often but not always organized as <a class="localLink" href="/Project">Project</a>s. Long-lived projects are sometimes sponsored by a variety of grants over time, but it is also common for a project to be associated with a single grant.<br/><br/>  The amount of a <a class="localLink" href="/Grant">Grant</a> is represented using <a class="localLink" href="/amount">amount</a> as a <a class="localLink" href="/MonetaryAmount">MonetaryAmount</a>. * EXTEND FROM: https://schema.org/Intangible * LOOK ALSO: https://schema.org/MonetaryGrant
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Grant {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<GrantOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct GrantOptions {
    /// An additional type for the item, typically used for adding more specific types from external vocabularies in microdata syntax. This is a relationship between something and a class that the thing is in. In RDFa syntax, it is better to use the native RDFa syntax - the 'typeof' attribute - for multiple types. Schema.org tools may have only weaker understanding of extra types, in particular those defined externally.
    pub additional_type: Option<URL>,

    /// An alias for the item.
    pub alternate_name: Option<Text>,

    /// A description of the item.
    pub description: Option<Text>,

    /// A sub property of description. A short description of the item used to disambiguate from other, similar items. Information from other properties (in particular, name) may be necessary for the description to be useful for disambiguation.
    pub disambiguating_description: Option<Text>,

    /// The identifier property represents any kind of identifier for any kind of <a class="localLink" href="/Thing">Thing</a>, such as ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing many of these, either as textual strings or as URL (URI) links. See <a href="/docs/datamodel.html#identifierBg">background notes</a> for more details.
    pub identifier: Option<identifier>,

    /// An image of the item. This can be a <a class="localLink" href="/URL">URL</a> or a fully described <a class="localLink" href="/ImageObject">ImageObject</a>.
    pub image: Option<image>,

    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being described. See <a href="/docs/datamodel.html#mainEntityBackground">background notes</a> for details.
    pub main_entity_of_page: Option<mainEntityOfPage>,

    /// The name of the item.
    pub name: Option<Text>,

    /// Indicates a potential Action, which describes an idealized action in which this thing would play an 'object' role.
    pub potential_action: Option<Action>,

    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL of the item's Wikipedia page, Wikidata entry, or official website.
    pub same_as: Option<URL>,

    /// A CreativeWork or Event about this Thing.
    pub subject_of: Option<subjectOf>,

    /// URL of the item.
    pub url: Option<URL>,

    /// Indicates something directly or indirectly funded or sponsored through a <a class="localLink" href="/Grant">Grant</a>. See also <a class="localLink" href="/ownershipFundingInfo">ownershipFundingInfo</a>.
    pub funded_item: Option<fundedItem>,

    /// A person or organization that supports (sponsors) something through some kind of financial contribution.
    pub funder: Option<funder>,

    /// A person or organization that supports a thing through a pledge, promise, or financial contribution. E.g. a sponsor of a Medical Study or a corporate sponsor of an event.
    pub sponsor: Option<sponsor>,
}

impl Grant {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
