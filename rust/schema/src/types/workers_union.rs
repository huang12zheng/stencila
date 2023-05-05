// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::actionable_feedback_policy_prop_enum::ActionableFeedbackPolicyPropEnum;
use super::address_prop_enum::AddressPropEnum;
use super::aggregate_rating::AggregateRating;
use super::area_served_prop_enum::AreaServedPropEnum;
use super::brand_prop_enum::BrandPropEnum;
use super::contact_point::ContactPoint;
use super::corrections_policy_prop_enum::CorrectionsPolicyPropEnum;
use super::date::Date;
use super::demand::Demand;
use super::diversity_policy_prop_enum::DiversityPolicyPropEnum;
use super::diversity_staffing_report_prop_enum::DiversityStaffingReportPropEnum;
use super::educational_occupational_credential::EducationalOccupationalCredential;
use super::ethics_policy_prop_enum::EthicsPolicyPropEnum;
use super::event::Event;
use super::funder_prop_enum::FunderPropEnum;
use super::grant::Grant;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::interaction_counter::InteractionCounter;
use super::keywords_prop_enum::KeywordsPropEnum;
use super::knows_about_prop_enum::KnowsAboutPropEnum;
use super::knows_language_prop_enum::KnowsLanguagePropEnum;
use super::location_prop_enum::LocationPropEnum;
use super::logo_prop_enum::LogoPropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::member_of_prop_enum::MemberOfPropEnum;
use super::member_prop_enum::MemberPropEnum;
use super::merchant_return_policy::MerchantReturnPolicy;
use super::nonprofit_type::NonprofitType;
use super::offer::Offer;
use super::offer_catalog::OfferCatalog;
use super::organization::Organization;
use super::ownership_funding_info_prop_enum::OwnershipFundingInfoPropEnum;
use super::owns_prop_enum::OwnsPropEnum;
use super::person::Person;
use super::place::Place;
use super::publishing_principles_prop_enum::PublishingPrinciplesPropEnum;
use super::quantitative_value::QuantitativeValue;
use super::review::Review;
use super::sponsor_prop_enum::SponsorPropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;
use super::unnamed_sources_policy_prop_enum::UnnamedSourcesPolicyPropEnum;

/// https://schema.org/WorkersUnion
/// * COMMENT:
/// A Workers Union (also known as a Labor Union, Labour Union, or Trade Union) is an organization that promotes the interests of its worker members by collectively bargaining with management, organizing, and political lobbying.
/// * EXTEND FROM:
/// https://schema.org/Organization
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct WorkersUnion {
    /// An additional type for the item, typically used for adding more specific types from external vocabularies in microdata syntax. This is a relationship between something and a class that the thing is in. In RDFa syntax, it is better to use the native RDFa syntax - the 'typeof' attribute - for multiple types. Schema.org tools may have only weaker understanding of extra types, in particular those defined externally.
    pub additional_type_prop_enum: Option<URL>,

    /// An alias for the item.
    pub alternate_name_prop_enum: Option<Text>,

    /// A description of the item.
    pub description_prop_enum: Option<Text>,

    /// A sub property of description. A short description of the item used to disambiguate from other, similar items. Information from other properties (in particular, name) may be necessary for the description to be useful for disambiguation.
    pub disambiguating_description_prop_enum: Option<Text>,

    /// The identifier property represents any kind of identifier for any kind of <a class="localLink" href="/Thing">Thing</a>, such as ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing many of these, either as textual strings or as URL (URI) links. See <a href="/docs/datamodel.html#identifierBg">background notes</a> for more details.
    pub identifier_prop_enum: Option<IdentifierPropEnum>,

    /// An image of the item. This can be a <a class="localLink" href="/URL">URL</a> or a fully described <a class="localLink" href="/ImageObject">ImageObject</a>.
    pub image_prop_enum: Option<ImagePropEnum>,

    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being described. See <a href="/docs/datamodel.html#mainEntityBackground">background notes</a> for details.
    pub main_entity_of_page_prop_enum: Option<MainEntityOfPagePropEnum>,

    /// The name of the item.
    pub name_prop_enum: Option<Text>,

    /// Indicates a potential Action, which describes an idealized action in which this thing would play an 'object' role.
    pub potential_action_prop_enum: Option<Action>,

    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL of the item's Wikipedia page, Wikidata entry, or official website.
    pub same_as_prop_enum: Option<URL>,

    /// A CreativeWork or Event about this Thing.
    pub subject_of_prop_enum: Option<SubjectOfPropEnum>,

    /// URL of the item.
    pub url_prop_enum: Option<URL>,

    /// For a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a> or other news-related <a class="localLink" href="/Organization">Organization</a>, a statement about public engagement activities (for news media, the newsroom’s), including involving the public - digitally or otherwise -- in coverage decisions, reporting and activities after publication.
    pub actionable_feedback_policy_prop_enum: Option<ActionableFeedbackPolicyPropEnum>,

    /// Physical address of the item.
    pub address_prop_enum: Option<AddressPropEnum>,

    /// The overall rating, based on a collection of reviews or ratings, of the item.
    pub aggregate_rating_prop_enum: Option<AggregateRating>,

    /// Alumni of an organization.
    pub alumni_prop_enum: Option<Person>,

    /// The geographic area where a service or offered item is provided.
    pub area_served_prop_enum: Option<AreaServedPropEnum>,

    /// An award won by or for this item.
    pub award_prop_enum: Option<Text>,

    /// The brand(s) associated with a product or service, or the brand(s) maintained by an organization or business person.
    pub brand_prop_enum: Option<BrandPropEnum>,

    /// A contact point for a person or organization.
    pub contact_point_prop_enum: Option<ContactPoint>,

    /// For an <a class="localLink" href="/Organization">Organization</a> (e.g. <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>), a statement describing (in news media, the newsroom’s) disclosure and correction policy for errors.
    pub corrections_policy_prop_enum: Option<CorrectionsPolicyPropEnum>,

    /// A relationship between an organization and a department of that organization, also described as an organization (allowing different urls, logos, opening hours). For example: a store with a pharmacy, or a bakery with a cafe.
    pub department_prop_enum: Option<Organization>,

    /// The date that this organization was dissolved.
    pub dissolution_date_prop_enum: Option<Date>,

    /// Statement on diversity policy by an <a class="localLink" href="/Organization">Organization</a> e.g. a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>. For a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>, a statement describing the newsroom’s diversity policy on both staffing and sources, typically providing staffing data.
    pub diversity_policy_prop_enum: Option<DiversityPolicyPropEnum>,

    /// For an <a class="localLink" href="/Organization">Organization</a> (often but not necessarily a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>), a report on staffing diversity issues. In a news context this might be for example ASNE or RTDNA (US) reports, or self-reported.
    pub diversity_staffing_report_prop_enum: Option<DiversityStaffingReportPropEnum>,

    /// The Dun &amp; Bradstreet DUNS number for identifying an organization or business person.
    pub duns_prop_enum: Option<Text>,

    /// Email address.
    pub email_prop_enum: Option<Text>,

    /// Someone working for this organization.
    pub employee_prop_enum: Option<Person>,

    /// Statement about ethics policy, e.g. of a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a> regarding journalistic and publishing practices, or of a <a class="localLink" href="/Restaurant">Restaurant</a>, a page describing food source policies. In the case of a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>, an ethicsPolicy is typically a statement describing the personal, organizational, and corporate standards of behavior expected by the organization.
    pub ethics_policy_prop_enum: Option<EthicsPolicyPropEnum>,

    /// Upcoming or past event associated with this place, organization, or action.
    pub event_prop_enum: Option<Event>,

    /// The fax number.
    pub fax_number_prop_enum: Option<Text>,

    /// A person who founded this organization.
    pub founder_prop_enum: Option<Person>,

    /// The date that this organization was founded.
    pub founding_date_prop_enum: Option<Date>,

    /// The place where the Organization was founded.
    pub founding_location_prop_enum: Option<Place>,

    /// A person or organization that supports (sponsors) something through some kind of financial contribution.
    pub funder_prop_enum: Option<FunderPropEnum>,

    /// A <a class="localLink" href="/Grant">Grant</a> that directly or indirectly provide funding or sponsorship for this item. See also <a class="localLink" href="/ownershipFundingInfo">ownershipFundingInfo</a>.
    pub funding_prop_enum: Option<Grant>,

    /// The <a href="http://www.gs1.org/gln">Global Location Number</a> (GLN, sometimes also referred to as International Location Number or ILN) of the respective organization, person, or place. The GLN is a 13-digit number used to identify parties and physical locations.
    pub global_location_number_prop_enum: Option<Text>,

    /// A credential awarded to the Person or Organization.
    pub has_credential_prop_enum: Option<EducationalOccupationalCredential>,

    /// Specifies a MerchantReturnPolicy that may be applicable.
    pub has_merchant_return_policy_prop_enum: Option<MerchantReturnPolicy>,

    /// Indicates an OfferCatalog listing for this Organization, Person, or Service.
    pub has_offer_catalog_prop_enum: Option<OfferCatalog>,

    /// Points-of-Sales operated by the organization or person.
    pub has_pos_prop_enum: Option<Place>,

    /// The number of interactions for the CreativeWork using the WebSite or SoftwareApplication. The most specific child type of InteractionCounter should be used.
    pub interaction_statistic_prop_enum: Option<InteractionCounter>,

    /// The International Standard of Industrial Classification of All Economic Activities (ISIC), Revision 4 code for a particular organization, business person, or place.
    pub isic_v4_prop_enum: Option<Text>,

    /// An organization identifier as defined in ISO 6523(-1). Note that many existing organization identifiers such as <a href="http://schema.org/leiCode">leiCode</a>, <a href="http://schema.org/duns">duns</a> and <a href="http://schema.org/vatID">vatID</a> can be expressed as an ISO 6523 identifier by setting the ICD part of the ISO 6523 identifier accordingly.
    pub iso_6523_code_prop_enum: Option<Text>,

    /// Keywords or tags used to describe some item. Multiple textual entries in a keywords list are typically delimited by commas, or by repeating the property.
    pub keywords_prop_enum: Option<KeywordsPropEnum>,

    /// Of a <a class="localLink" href="/Person">Person</a>, and less typically of an <a class="localLink" href="/Organization">Organization</a>, to indicate a topic that is known about - suggesting possible expertise but not implying it. We do not distinguish skill levels here, or relate this to educational content, events, objectives or <a class="localLink" href="/JobPosting">JobPosting</a> descriptions.
    pub knows_about_prop_enum: Option<KnowsAboutPropEnum>,

    /// Of a <a class="localLink" href="/Person">Person</a>, and less typically of an <a class="localLink" href="/Organization">Organization</a>, to indicate a known language. We do not distinguish skill levels or reading/writing/speaking/signing here. Use language codes from the <a href="http://tools.ietf.org/html/bcp47">IETF BCP 47 standard</a>.
    pub knows_language_prop_enum: Option<KnowsLanguagePropEnum>,

    /// The official name of the organization, e.g. the registered company name.
    pub legal_name_prop_enum: Option<Text>,

    /// An organization identifier that uniquely identifies a legal entity as defined in ISO 17442.
    pub lei_code_prop_enum: Option<Text>,

    /// The location of, for example, where an event is happening, where an organization is located, or where an action takes place.
    pub location_prop_enum: Option<LocationPropEnum>,

    /// An associated logo.
    pub logo_prop_enum: Option<LogoPropEnum>,

    /// A pointer to products or services offered by the organization or person.
    pub makes_offer_prop_enum: Option<Offer>,

    /// A member of an Organization or a ProgramMembership. Organizations can be members of organizations; ProgramMembership is typically for individuals.
    pub member_prop_enum: Option<MemberPropEnum>,

    /// An Organization (or ProgramMembership) to which this Person or Organization belongs.
    pub member_of_prop_enum: Option<MemberOfPropEnum>,

    /// The North American Industry Classification System (NAICS) code for a particular organization or business person.
    pub naics_prop_enum: Option<Text>,

    /// nonprofitStatus indicates the legal status of a non-profit organization in its primary place of business.
    pub nonprofit_status_prop_enum: Option<NonprofitType>,

    /// The number of employees in an organization, e.g. business.
    pub number_of_employees_prop_enum: Option<QuantitativeValue>,

    /// For an <a class="localLink" href="/Organization">Organization</a> (often but not necessarily a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>), a description of organizational ownership structure; funding and grants. In a news/media setting, this is with particular reference to editorial independence.   Note that the <a class="localLink" href="/funder">funder</a> is also available and can be used to make basic funder information machine-readable.
    pub ownership_funding_info_prop_enum: Option<OwnershipFundingInfoPropEnum>,

    /// Products owned by the organization or person.
    pub owns_prop_enum: Option<OwnsPropEnum>,

    /// The larger organization that this organization is a <a class="localLink" href="/subOrganization">subOrganization</a> of, if any.
    pub parent_organization_prop_enum: Option<Organization>,

    /// The publishingPrinciples property indicates (typically via <a class="localLink" href="/URL">URL</a>) a document describing the editorial principles of an <a class="localLink" href="/Organization">Organization</a> (or individual, e.g. a <a class="localLink" href="/Person">Person</a> writing a blog) that relate to their activities as a publisher, e.g. ethics or diversity policies. When applied to a <a class="localLink" href="/CreativeWork">CreativeWork</a> (e.g. <a class="localLink" href="/NewsArticle">NewsArticle</a>) the principles are those of the party primarily responsible for the creation of the <a class="localLink" href="/CreativeWork">CreativeWork</a>.<br/><br/>  While such policies are most typically expressed in natural language, sometimes related information (e.g. indicating a <a class="localLink" href="/funder">funder</a>) can be expressed using schema.org terminology.
    pub publishing_principles_prop_enum: Option<PublishingPrinciplesPropEnum>,

    /// A review of the item.
    pub review_prop_enum: Option<Review>,

    /// A pointer to products or services sought by the organization or person (demand).
    pub seeks_prop_enum: Option<Demand>,

    /// A slogan or motto associated with the item.
    pub slogan_prop_enum: Option<Text>,

    /// A person or organization that supports a thing through a pledge, promise, or financial contribution. E.g. a sponsor of a Medical Study or a corporate sponsor of an event.
    pub sponsor_prop_enum: Option<SponsorPropEnum>,

    /// A relationship between two organizations where the first includes the second, e.g., as a subsidiary. See also: the more specific 'department' property.
    pub sub_organization_prop_enum: Option<Organization>,

    /// The Tax / Fiscal ID of the organization or person, e.g. the TIN in the US or the CIF/NIF in Spain.
    pub tax_id_prop_enum: Option<Text>,

    /// The telephone number.
    pub telephone_prop_enum: Option<Text>,

    /// For an <a class="localLink" href="/Organization">Organization</a> (typically a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>), a statement about policy on use of unnamed sources and the decision process required.
    pub unnamed_sources_policy_prop_enum: Option<UnnamedSourcesPolicyPropEnum>,

    /// The Value-added Tax ID of the organization or person.
    pub vat_id_prop_enum: Option<Text>,
}

impl WorkersUnion {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
