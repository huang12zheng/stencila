// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::adult_oriented_enumeration::AdultOrientedEnumeration;
use super::aggregate_rating::AggregateRating;
use super::alignment_object::AlignmentObject;
use super::audience::Audience;
use super::boolean::Boolean;
use super::claim::Claim;
use super::comment::Comment;
use super::country::Country;
use super::creative_work::CreativeWork;
use super::date::Date;
use super::date_time::DateTime;
use super::duration::Duration;
use super::energy_consumption_details::EnergyConsumptionDetails;
use super::event::Event;
use super::grant::Grant;
use super::integer::Integer;
use super::interaction_counter::InteractionCounter;
use super::item_list::ItemList;
use super::media_object::MediaObject;
use super::merchant_return_policy::MerchantReturnPolicy;
use super::number::Number;
use super::offer_item_condition::OfferItemCondition;
use super::organization::Organization;
use super::person::Person;
use super::place::Place;
use super::product::Product;
use super::property_value::PropertyValue;
use super::publication_event::PublicationEvent;
use super::quantitative_value::QuantitativeValue;
use super::review::Review;
use super::text::Text;
use super::thing::Thing;
use super::type_and_quantity_node::TypeAndQuantityNode;
use super::url::URL;
use super::acquire_license_page::acquireLicensePage;
use super::archived_at::archivedAt;
use super::asin::asin;
use super::assesses::assesses;
use super::audio::audio;
use super::author::author;
use super::brand::brand;
use super::category::category;
use super::citation::citation;
use super::content_rating::contentRating;
use super::contributor::contributor;
use super::copyright_holder::copyrightHolder;
use super::correction::correction;
use super::creative_work_status::creativeWorkStatus;
use super::creator::creator;
use super::date_created::dateCreated;
use super::date_modified::dateModified;
use super::date_published::datePublished;
use super::depth::depth;
use super::edit_eidr::editEIDR;
use super::educational_level::educationalLevel;
use super::educational_use::educationalUse;
use super::encoding_format::encodingFormat;
use super::expires::expires;
use super::funder::funder;
use super::genre::genre;
use super::gtin::gtin;
use super::height::height;
use super::identifier::identifier;
use super::image::image;
use super::in_language::inLanguage;
use super::is_based_on::isBasedOn;
use super::is_part_of::isPartOf;
use super::is_related_to::isRelatedTo;
use super::is_similar_to::isSimilarTo;
use super::is_variant_of::isVariantOf;
use super::keywords::keywords;
use super::learning_resource_type::learningResourceType;
use super::license::license;
use super::logo::logo;
use super::main_entity_of_page::mainEntityOfPage;
use super::maintainer::maintainer;
use super::material::material;
use super::material_extent::materialExtent;
use super::model::model;
use super::negative_notes::negativeNotes;
use super::offers::offers;
use super::pattern::pattern;
use super::position::position;
use super::positive_notes::positiveNotes;
use super::producer::producer;
use super::provider::provider;
use super::publisher::publisher;
use super::publishing_principles::publishingPrinciples;
use super::schema_version::schemaVersion;
use super::sd_license::sdLicense;
use super::sd_publisher::sdPublisher;
use super::size::size;
use super::sponsor::sponsor;
use super::subject_of::subjectOf;
use super::teaches::teaches;
use super::temporal::temporal;
use super::temporal_coverage::temporalCoverage;
use super::translator::translator;
use super::usage_info::usageInfo;
use super::version::version;
use super::video::video;
use super::width::width;

/// * MOD OF: https://pending.schema.org * COMMENT: A set of products (either <a class="localLink" href="/ProductGroup">ProductGroup</a>s or specific variants) that are listed together e.g. in an <a class="localLink" href="/Offer">Offer</a>. * EXTEND FROM: https://schema.org/Collection, https://schema.org/Product
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ProductCollection {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<ProductCollectionOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ProductCollectionOptions {
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

    /// The subject matter of the content.
    pub about: Option<Thing>,

    /// An abstract is a short description that summarizes a <a class="localLink" href="/CreativeWork">CreativeWork</a>.
    pub r_abstract: Option<Text>,

    /// The human sensory perceptual system or cognitive faculty through which a person may process or perceive information. Values should be drawn from the <a href="https://www.w3.org/2021/a11y-discov-vocab/latest/#accessMode-vocabulary">approved vocabulary</a>.
    pub access_mode: Option<Text>,

    /// A list of single or combined accessModes that are sufficient to understand all the intellectual content of a resource. Values should be drawn from the <a href="https://www.w3.org/2021/a11y-discov-vocab/latest/#accessModeSufficient-vocabulary">approved vocabulary</a>.
    pub access_mode_sufficient: Option<ItemList>,

    /// Indicates that the resource is compatible with the referenced accessibility API. Values should be drawn from the <a href="https://www.w3.org/2021/a11y-discov-vocab/latest/#accessibilityAPI-vocabulary">approved vocabulary</a>.
    pub accessibility_api: Option<Text>,

    /// Identifies input methods that are sufficient to fully control the described resource. Values should be drawn from the <a href="https://www.w3.org/2021/a11y-discov-vocab/latest/#accessibilityControl-vocabulary">approved vocabulary</a>.
    pub accessibility_control: Option<Text>,

    /// Content features of the resource, such as accessible media, alternatives and supported enhancements for accessibility. Values should be drawn from the <a href="https://www.w3.org/2021/a11y-discov-vocab/latest/#accessibilityFeature-vocabulary">approved vocabulary</a>.
    pub accessibility_feature: Option<Text>,

    /// A characteristic of the described resource that is physiologically dangerous to some users. Related to WCAG 2.0 guideline 2.3. Values should be drawn from the <a href="https://www.w3.org/2021/a11y-discov-vocab/latest/#accessibilityHazard-vocabulary">approved vocabulary</a>.
    pub accessibility_hazard: Option<Text>,

    /// A human-readable summary of specific accessibility features or deficiencies, consistent with the other accessibility metadata but expressing subtleties such as "short descriptions are present but long descriptions will be needed for non-visual users" or "short descriptions are present and no long descriptions are needed."
    pub accessibility_summary: Option<Text>,

    /// Specifies the Person that is legally accountable for the CreativeWork.
    pub accountable_person: Option<Person>,

    /// Indicates a page documenting how licenses can be purchased or otherwise acquired, for the current item.
    pub acquire_license_page: Option<acquireLicensePage>,

    /// The overall rating, based on a collection of reviews or ratings, of the item.
    pub aggregate_rating: Option<AggregateRating>,

    /// A secondary title of the CreativeWork.
    pub alternative_headline: Option<Text>,

    /// Indicates a page or other link involved in archival of a <a class="localLink" href="/CreativeWork">CreativeWork</a>. In the case of <a class="localLink" href="/MediaReview">MediaReview</a>, the items in a <a class="localLink" href="/MediaReviewItem">MediaReviewItem</a> may often become inaccessible, but be archived by archival, journalistic, activist, or law enforcement organizations. In such cases, the referenced page may not directly publish the content.
    pub archived_at: Option<archivedAt>,

    /// The item being described is intended to assess the competency or learning outcome defined by the referenced term.
    pub assesses: Option<assesses>,

    /// A media object that encodes this CreativeWork. This property is a synonym for encoding.
    pub associated_media: Option<MediaObject>,

    /// An intended audience, i.e. a group for whom something was created.
    pub audience: Option<Audience>,

    /// An embedded audio object.
    pub audio: Option<audio>,

    /// The author of this content or rating. Please note that author is special in that HTML 5 provides a special mechanism for indicating authorship via the rel tag. That is equivalent to this and may be used interchangeably.
    pub author: Option<author>,

    /// An award won by or for this item.
    pub award: Option<Text>,

    /// Fictional person connected with a creative work.
    pub character: Option<Person>,

    /// A citation or reference to another creative work, such as another publication, web page, scholarly article, etc.
    pub citation: Option<citation>,

    /// Comments, typically from users.
    pub comment: Option<Comment>,

    /// The number of comments this CreativeWork (e.g. Article, Question or Answer) has received. This is most applicable to works published in Web sites with commenting system; additional comments may exist elsewhere.
    pub comment_count: Option<Integer>,

    /// Conditions that affect the availability of, or method(s) of access to, an item. Typically used for real world items such as an <a class="localLink" href="/ArchiveComponent">ArchiveComponent</a> held by an <a class="localLink" href="/ArchiveOrganization">ArchiveOrganization</a>. This property is not suitable for use as a general Web access control mechanism. It is expressed only in natural language.<br/><br/>  For example "Available by appointment from the Reading Room" or "Accessible only from logged-in accounts ".
    pub conditions_of_access: Option<Text>,

    /// The location depicted or described in the content. For example, the location in a photograph or painting.
    pub content_location: Option<Place>,

    /// Official rating of a piece of content&#x2014;for example, 'MPAA PG-13'.
    pub content_rating: Option<contentRating>,

    /// The specific time described by a creative work, for works (e.g. articles, video objects etc.) that emphasise a particular moment within an Event.
    pub content_reference_time: Option<DateTime>,

    /// A secondary contributor to the CreativeWork or Event.
    pub contributor: Option<contributor>,

    /// The party holding the legal copyright to the CreativeWork.
    pub copyright_holder: Option<copyrightHolder>,

    /// Text of a notice appropriate for describing the copyright aspects of this Creative Work, ideally indicating the owner of the copyright for the Work.
    pub copyright_notice: Option<Text>,

    /// The year during which the claimed copyright for the CreativeWork was first asserted.
    pub copyright_year: Option<Number>,

    /// Indicates a correction to a <a class="localLink" href="/CreativeWork">CreativeWork</a>, either via a <a class="localLink" href="/CorrectionComment">CorrectionComment</a>, textually or in another document.
    pub correction: Option<correction>,

    /// The country of origin of something, including products as well as creative  works such as movie and TV content.<br/><br/>  In the case of TV and movie, this would be the country of the principle offices of the production company or individual responsible for the movie. For other kinds of <a class="localLink" href="/CreativeWork">CreativeWork</a> it is difficult to provide fully general guidance, and properties such as <a class="localLink" href="/contentLocation">contentLocation</a> and <a class="localLink" href="/locationCreated">locationCreated</a> may be more applicable.<br/><br/>  In the case of products, the country of origin of the product. The exact interpretation of this may vary by context and product type, and cannot be fully enumerated here.
    pub country_of_origin: Option<Country>,

    /// The status of a creative work in terms of its stage in a lifecycle. Example terms include Incomplete, Draft, Published, Obsolete. Some organizations define a set of terms for the stages of their publication lifecycle.
    pub creative_work_status: Option<creativeWorkStatus>,

    /// The creator/author of this CreativeWork. This is the same as the Author property for CreativeWork.
    pub creator: Option<creator>,

    /// Text that can be used to credit person(s) and/or organization(s) associated with a published Creative Work.
    pub credit_text: Option<Text>,

    /// The date on which the CreativeWork was created or the item was added to a DataFeed.
    pub date_created: Option<dateCreated>,

    /// The date on which the CreativeWork was most recently modified or when the item's entry was modified within a DataFeed.
    pub date_modified: Option<dateModified>,

    /// Date of first broadcast/publication.
    pub date_published: Option<datePublished>,

    /// A link to the page containing the comments of the CreativeWork.
    pub discussion_url: Option<URL>,

    /// An <a href="https://eidr.org/">EIDR</a> (Entertainment Identifier Registry) <a class="localLink" href="/identifier">identifier</a> representing a specific edit / edition for a work of film or television.<br/><br/>  For example, the motion picture known as "Ghostbusters" whose <a class="localLink" href="/titleEIDR">titleEIDR</a> is "10.5240/7EC7-228A-510A-053E-CBB8-J" has several edits, e.g. "10.5240/1F2A-E1C5-680A-14C6-E76B-I" and "10.5240/8A35-3BEE-6497-5D12-9E4F-3".<br/><br/>  Since schema.org types like <a class="localLink" href="/Movie">Movie</a> and <a class="localLink" href="/TVEpisode">TVEpisode</a> can be used for both works and their multiple expressions, it is possible to use <a class="localLink" href="/titleEIDR">titleEIDR</a> alone (for a general description), or alongside <a class="localLink" href="/editEIDR">editEIDR</a> for a more edit-specific description.
    pub edit_eidr: Option<editEIDR>,

    /// Specifies the Person who edited the CreativeWork.
    pub editor: Option<Person>,

    /// An alignment to an established educational framework.<br/><br/>  This property should not be used where the nature of the alignment can be described using a simple property, for example to express that a resource <a class="localLink" href="/teaches">teaches</a> or <a class="localLink" href="/assesses">assesses</a> a competency.
    pub educational_alignment: Option<AlignmentObject>,

    /// The level in terms of progression through an educational or training context. Examples of educational levels include 'beginner', 'intermediate' or 'advanced', and formal sets of level indicators.
    pub educational_level: Option<educationalLevel>,

    /// The purpose of a work in the context of education; for example, 'assignment', 'group work'.
    pub educational_use: Option<educationalUse>,

    /// A media object that encodes this CreativeWork. This property is a synonym for associatedMedia.
    pub encoding: Option<MediaObject>,

    /// Media type typically expressed using a MIME format (see <a href="http://www.iana.org/assignments/media-types/media-types.xhtml">IANA site</a> and <a href="https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types">MDN reference</a>), e.g. application/zip for a SoftwareApplication binary, audio/mpeg for .mp3 etc.<br/><br/>  In cases where a <a class="localLink" href="/CreativeWork">CreativeWork</a> has several media type representations, <a class="localLink" href="/encoding">encoding</a> can be used to indicate each <a class="localLink" href="/MediaObject">MediaObject</a> alongside particular <a class="localLink" href="/encodingFormat">encodingFormat</a> information.<br/><br/>  Unregistered or niche encoding and file formats can be indicated instead via the most appropriate URL, e.g. defining Web page or a Wikipedia/Wikidata entry.
    pub encoding_format: Option<encodingFormat>,

    /// A creative work that this work is an example/instance/realization/derivation of.
    pub example_of_work: Option<CreativeWork>,

    /// Date the content expires and is no longer useful or available. For example a <a class="localLink" href="/VideoObject">VideoObject</a> or <a class="localLink" href="/NewsArticle">NewsArticle</a> whose availability or relevance is time-limited, or a <a class="localLink" href="/ClaimReview">ClaimReview</a> fact check whose publisher wants to indicate that it may no longer be relevant (or helpful to highlight) after some date.
    pub expires: Option<expires>,

    /// A person or organization that supports (sponsors) something through some kind of financial contribution.
    pub funder: Option<funder>,

    /// A <a class="localLink" href="/Grant">Grant</a> that directly or indirectly provide funding or sponsorship for this item. See also <a class="localLink" href="/ownershipFundingInfo">ownershipFundingInfo</a>.
    pub funding: Option<Grant>,

    /// Genre of the creative work, broadcast channel or group.
    pub genre: Option<genre>,

    /// Indicates an item or CreativeWork that is part of this item, or CreativeWork (in some sense).
    pub has_part: Option<CreativeWork>,

    /// Headline of the article.
    pub headline: Option<Text>,

    /// The language of the content or performance or used in an action. Please use one of the language codes from the <a href="http://tools.ietf.org/html/bcp47">IETF BCP 47 standard</a>. See also <a class="localLink" href="/availableLanguage">availableLanguage</a>.
    pub in_language: Option<inLanguage>,

    /// The number of interactions for the CreativeWork using the WebSite or SoftwareApplication. The most specific child type of InteractionCounter should be used.
    pub interaction_statistic: Option<InteractionCounter>,

    /// The predominant mode of learning supported by the learning resource. Acceptable values are 'active', 'expositive', or 'mixed'.
    pub interactivity_type: Option<Text>,

    /// Used to indicate a specific claim contained, implied, translated or refined from the content of a <a class="localLink" href="/MediaObject">MediaObject</a> or other <a class="localLink" href="/CreativeWork">CreativeWork</a>. The interpreting party can be indicated using <a class="localLink" href="/claimInterpreter">claimInterpreter</a>.
    pub interpreted_as_claim: Option<Claim>,

    /// A flag to signal that the item, event, or place is accessible for free.
    pub is_accessible_for_free: Option<Boolean>,

    /// A resource from which this work is derived or from which it is a modification or adaption.
    pub is_based_on: Option<isBasedOn>,

    /// Indicates whether this content is family friendly.
    pub is_family_friendly: Option<Boolean>,

    /// Indicates an item or CreativeWork that this item, or CreativeWork (in some sense), is part of.
    pub is_part_of: Option<Box<isPartOf>>,

    /// Keywords or tags used to describe some item. Multiple textual entries in a keywords list are typically delimited by commas, or by repeating the property.
    pub keywords: Option<keywords>,

    /// The predominant type or kind characterizing the learning resource. For example, 'presentation', 'handout'.
    pub learning_resource_type: Option<learningResourceType>,

    /// A license document that applies to this content, typically indicated by URL.
    pub license: Option<license>,

    /// The location where the CreativeWork was created, which may not be the same as the location depicted in the CreativeWork.
    pub location_created: Option<Place>,

    /// Indicates the primary entity described in some page or other CreativeWork.
    pub main_entity: Option<Thing>,

    /// A maintainer of a <a class="localLink" href="/Dataset">Dataset</a>, software package (<a class="localLink" href="/SoftwareApplication">SoftwareApplication</a>), or other <a class="localLink" href="/Project">Project</a>. A maintainer is a <a class="localLink" href="/Person">Person</a> or <a class="localLink" href="/Organization">Organization</a> that manages contributions to, and/or publication of, some (typically complex) artifact. It is common for distributions of software and data to be based on "upstream" sources. When <a class="localLink" href="/maintainer">maintainer</a> is applied to a specific version of something e.g. a particular version or packaging of a <a class="localLink" href="/Dataset">Dataset</a>, it is always  possible that the upstream source has a different maintainer. The <a class="localLink" href="/isBasedOn">isBasedOn</a> property can be used to indicate such relationships between datasets to make the different maintenance roles clear. Similarly in the case of software, a package may have dedicated maintainers working on integration into software distributions such as Ubuntu, as well as upstream maintainers of the underlying work.
    pub maintainer: Option<maintainer>,

    /// A material that something is made from, e.g. leather, wool, cotton, paper.
    pub material: Option<material>,

    /// The quantity of the materials being described or an expression of the physical space they occupy.
    pub material_extent: Option<materialExtent>,

    /// Indicates that the CreativeWork contains a reference to, but is not necessarily about a concept.
    pub mentions: Option<Thing>,

    /// An offer to provide this item&#x2014;for example, an offer to sell a product, rent the DVD of a movie, perform a service, or give away tickets to an event. Use <a class="localLink" href="/businessFunction">businessFunction</a> to indicate the kind of transaction offered, i.e. sell, lease, etc. This property can also be used to describe a <a class="localLink" href="/Demand">Demand</a>. While this property is listed as expected on a number of common types, it can be used in others. In that case, using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
    pub offers: Option<offers>,

    /// A pattern that something has, for example 'polka dot', 'striped', 'Canadian flag'. Values are typically expressed as text, although links to controlled value schemes are also supported.
    pub pattern: Option<pattern>,

    /// The position of an item in a series or sequence of items.
    pub position: Option<position>,

    /// The person or organization who produced the work (e.g. music album, movie, TV/radio series etc.).
    pub producer: Option<producer>,

    /// The service provider, service operator, or service performer; the goods producer. Another party (a seller) may offer those services or goods on behalf of the provider. A provider may also serve as the seller.
    pub provider: Option<provider>,

    /// A publication event associated with the item.
    pub publication: Option<PublicationEvent>,

    /// The publisher of the creative work.
    pub publisher: Option<publisher>,

    /// The publishing division which published the comic.
    pub publisher_imprint: Option<Organization>,

    /// The publishingPrinciples property indicates (typically via <a class="localLink" href="/URL">URL</a>) a document describing the editorial principles of an <a class="localLink" href="/Organization">Organization</a> (or individual, e.g. a <a class="localLink" href="/Person">Person</a> writing a blog) that relate to their activities as a publisher, e.g. ethics or diversity policies. When applied to a <a class="localLink" href="/CreativeWork">CreativeWork</a> (e.g. <a class="localLink" href="/NewsArticle">NewsArticle</a>) the principles are those of the party primarily responsible for the creation of the <a class="localLink" href="/CreativeWork">CreativeWork</a>.<br/><br/>  While such policies are most typically expressed in natural language, sometimes related information (e.g. indicating a <a class="localLink" href="/funder">funder</a>) can be expressed using schema.org terminology.
    pub publishing_principles: Option<publishingPrinciples>,

    /// The Event where the CreativeWork was recorded. The CreativeWork may capture all or part of the event.
    pub recorded_at: Option<Event>,

    /// The place and time the release was issued, expressed as a PublicationEvent.
    pub released_event: Option<PublicationEvent>,

    /// A review of the item.
    pub review: Option<Review>,

    /// Indicates (by URL or string) a particular version of a schema used in some CreativeWork. This property was created primarily to     indicate the use of a specific schema.org release, e.g. <code>10.0</code> as a simple string, or more explicitly via URL, <code>http://schema.org/docs/releases.html#v10.0</code>. There may be situations in which other schemas might usefully be referenced this way, e.g. <code>http://dublincore.org/specifications/dublin-core/dces/1999-07-02/</code> but this has not been carefully explored in the community.
    pub schema_version: Option<schemaVersion>,

    /// Indicates the date on which the current structured data was generated / published. Typically used alongside <a class="localLink" href="/sdPublisher">sdPublisher</a>
    pub sd_date_published: Option<Date>,

    /// A license document that applies to this structured data, typically indicated by URL.
    pub sd_license: Option<sdLicense>,

    /// Indicates the party responsible for generating and publishing the current structured data markup, typically in cases where the structured data is derived automatically from existing published content but published on a different site. For example, student projects and open data initiatives often re-publish existing content with more explicitly structured metadata. The <a class="localLink" href="/sdPublisher">sdPublisher</a> property helps make such practices more explicit.
    pub sd_publisher: Option<sdPublisher>,

    /// A standardized size of a product or creative work, specified either through a simple textual string (for example 'XL', '32Wx34L'), a  QuantitativeValue with a unitCode, or a comprehensive and structured <a class="localLink" href="/SizeSpecification">SizeSpecification</a>; in other cases, the <a class="localLink" href="/width">width</a>, <a class="localLink" href="/height">height</a>, <a class="localLink" href="/depth">depth</a> and <a class="localLink" href="/weight">weight</a> properties may be more applicable.
    pub size: Option<size>,

    /// The Organization on whose behalf the creator was working.
    pub source_organization: Option<Organization>,

    /// The "spatial" property can be used in cases when more specific properties (e.g. <a class="localLink" href="/locationCreated">locationCreated</a>, <a class="localLink" href="/spatialCoverage">spatialCoverage</a>, <a class="localLink" href="/contentLocation">contentLocation</a>) are not known to be appropriate.
    pub spatial: Option<Place>,

    /// The spatialCoverage of a CreativeWork indicates the place(s) which are the focus of the content. It is a subproperty of       contentLocation intended primarily for more technical and detailed materials. For example with a Dataset, it indicates       areas that the dataset describes: a dataset of New York weather would have spatialCoverage which was the place: the state of New York.
    pub spatial_coverage: Option<Place>,

    /// A person or organization that supports a thing through a pledge, promise, or financial contribution. E.g. a sponsor of a Medical Study or a corporate sponsor of an event.
    pub sponsor: Option<sponsor>,

    /// The item being described is intended to help a person learn the competency or learning outcome defined by the referenced term.
    pub teaches: Option<teaches>,

    /// The "temporal" property can be used in cases where more specific properties (e.g. <a class="localLink" href="/temporalCoverage">temporalCoverage</a>, <a class="localLink" href="/dateCreated">dateCreated</a>, <a class="localLink" href="/dateModified">dateModified</a>, <a class="localLink" href="/datePublished">datePublished</a>) are not known to be appropriate.
    pub temporal: Option<temporal>,

    /// The temporalCoverage of a CreativeWork indicates the period that the content applies to, i.e. that it describes, either as a DateTime or as a textual string indicating a time period in <a href="https://en.wikipedia.org/wiki/ISO_8601#Time_intervals">ISO 8601 time interval format</a>. In       the case of a Dataset it will typically indicate the relevant time period in a precise notation (e.g. for a 2011 census dataset, the year 2011 would be written "2011/2012"). Other forms of content, e.g. ScholarlyArticle, Book, TVSeries or TVEpisode, may indicate their temporalCoverage in broader terms - textually or via well-known URL.       Written works such as books may sometimes have precise temporal coverage too, e.g. a work set in 1939 - 1945 can be indicated in ISO 8601 interval format format via "1939/1945".<br/><br/>  Open-ended date ranges can be written with ".." in place of the end date. For example, "2015-11/.." indicates a range beginning in November 2015 and with no specified final date. This is tentative and might be updated in future when ISO 8601 is officially updated.
    pub temporal_coverage: Option<temporalCoverage>,

    /// The textual content of this CreativeWork.
    pub text: Option<Text>,

    /// A thumbnail image relevant to the Thing.
    pub thumbnail_url: Option<URL>,

    /// Approximate or typical time it takes to work with or through this learning resource for the typical intended target audience, e.g. 'PT30M', 'PT1H25M'.
    pub time_required: Option<Duration>,

    /// The work that this work has been translated from. E.g. 物种起源 is a translationOf “On the Origin of Species”.
    pub translation_of_work: Option<CreativeWork>,

    /// Organization or person who adapts a creative work to different languages, regional differences and technical requirements of a target market, or that translates during some event.
    pub translator: Option<translator>,

    /// The typical expected age range, e.g. '7-9', '11-'.
    pub typical_age_range: Option<Text>,

    /// The schema.org <a class="localLink" href="/usageInfo">usageInfo</a> property indicates further information about a <a class="localLink" href="/CreativeWork">CreativeWork</a>. This property is applicable both to works that are freely available and to those that require payment or other transactions. It can reference additional information, e.g. community expectations on preferred linking and citation conventions, as well as purchasing details. For something that can be commercially licensed, usageInfo can provide detailed, resource-specific information about licensing options.<br/><br/>  This property can be used alongside the license property which indicates license(s) applicable to some piece of content. The usageInfo property can provide information about other licensing options, e.g. acquiring commercial usage rights for an image that is also available under non-commercial creative commons licenses.
    pub usage_info: Option<usageInfo>,

    /// The version of the CreativeWork embodied by a specified resource.
    pub version: Option<version>,

    /// An embedded video object.
    pub video: Option<video>,

    /// Example/instance/realization/derivation of the concept of this creative work. E.g. the paperback edition, first edition, or e-book.
    pub work_example: Option<CreativeWork>,

    /// A work that is a translation of the content of this work. E.g. 西遊記 has an English workTranslation “Journey to the West”, a German workTranslation “Monkeys Pilgerfahrt” and a Vietnamese  translation Tây du ký bình khảo.
    pub work_translation: Option<CreativeWork>,

    /// The number of items in the <a class="localLink" href="/Collection">Collection</a>.
    pub collection_size: Option<Integer>,

    /// A property-value pair representing an additional characteristic of the entity, e.g. a product feature or another characteristic for which there is no matching property in schema.org.<br/><br/>  Note: Publishers should be aware that applications designed to use specific schema.org properties (e.g. http://schema.org/width, http://schema.org/color, http://schema.org/gtin13, ...) will typically expect such data to be provided using those properties, rather than using the generic property/value mechanism.
    pub additional_property: Option<PropertyValue>,

    /// An Amazon Standard Identification Number (ASIN) is a 10-character alphanumeric unique identifier assigned by Amazon.com and its partners for product identification within the Amazon organization (summary from <a href="https://en.wikipedia.org/wiki/Amazon_Standard_Identification_Number">Wikipedia</a>'s article).<br/><br/>  Note also that this is a definition for how to include ASINs in Schema.org data, and not a definition of ASINs in general - see documentation from Amazon for authoritative details. ASINs are most commonly encoded as text strings, but the [asin] property supports URL/URI as potential values too.
    pub asin: Option<asin>,

    /// The brand(s) associated with a product or service, or the brand(s) maintained by an organization or business person.
    pub brand: Option<brand>,

    /// A category for the item. Greater signs or slashes can be used to informally indicate a category hierarchy.
    pub category: Option<category>,

    /// The color of the product.
    pub color: Option<Text>,

    /// The place where the product was assembled.
    pub country_of_assembly: Option<Text>,

    /// The place where the item (typically <a class="localLink" href="/Product">Product</a>) was last processed and tested before importation.
    pub country_of_last_processing: Option<Text>,

    /// The depth of the item.
    pub depth: Option<depth>,

    /// A Global Trade Item Number (<a href="https://www.gs1.org/standards/id-keys/gtin">GTIN</a>). GTINs identify trade items, including products and services, using numeric identification codes.<br/><br/>  The GS1 <a href="https://www.gs1.org/standards/Digital-Link/">digital link specifications</a> express GTINs as URLs (URIs, IRIs, etc.). Details including regular expression examples can be found in, Section 6 of the GS1 URI Syntax specification; see also <a href="https://github.com/schemaorg/schemaorg/issues/3156#issuecomment-1209522809">schema.org tracking issue</a> for schema.org-specific discussion. A correct <a class="localLink" href="/gtin">gtin</a> value should be a valid GTIN, which means that it should be an all-numeric string of either 8, 12, 13 or 14 digits, or a "GS1 Digital Link" URL based on such a string. The numeric component should also have a <a href="https://www.gs1.org/services/check-digit-calculator">valid GS1 check digit</a> and meet the other rules for valid GTINs. See also <a href="http://www.gs1.org/barcodes/technical/idkeys/gtin">GS1's GTIN Summary</a> and <a href="https://en.wikipedia.org/wiki/Global_Trade_Item_Number">Wikipedia</a> for more details. Left-padding of the gtin values is not required or encouraged. The <a class="localLink" href="/gtin">gtin</a> property generalizes the earlier <a class="localLink" href="/gtin8">gtin8</a>, <a class="localLink" href="/gtin12">gtin12</a>, <a class="localLink" href="/gtin13">gtin13</a>, and <a class="localLink" href="/gtin14">gtin14</a> properties.<br/><br/>  Note also that this is a definition for how to include GTINs in Schema.org data, and not a definition of GTINs in general - see the GS1 documentation for authoritative details.
    pub gtin: Option<gtin>,

    /// The GTIN-12 code of the product, or the product to which the offer refers. The GTIN-12 is the 12-digit GS1 Identification Key composed of a U.P.C. Company Prefix, Item Reference, and Check Digit used to identify trade items. See <a href="http://www.gs1.org/barcodes/technical/idkeys/gtin">GS1 GTIN Summary</a> for more details.
    pub gtin_12: Option<Text>,

    /// The GTIN-13 code of the product, or the product to which the offer refers. This is equivalent to 13-digit ISBN codes and EAN UCC-13. Former 12-digit UPC codes can be converted into a GTIN-13 code by simply adding a preceding zero. See <a href="http://www.gs1.org/barcodes/technical/idkeys/gtin">GS1 GTIN Summary</a> for more details.
    pub gtin_13: Option<Text>,

    /// The GTIN-14 code of the product, or the product to which the offer refers. See <a href="http://www.gs1.org/barcodes/technical/idkeys/gtin">GS1 GTIN Summary</a> for more details.
    pub gtin_14: Option<Text>,

    /// The GTIN-8 code of the product, or the product to which the offer refers. This code is also known as EAN/UCC-8 or 8-digit EAN. See <a href="http://www.gs1.org/barcodes/technical/idkeys/gtin">GS1 GTIN Summary</a> for more details.
    pub gtin_8: Option<Text>,

    /// Used to tag an item to be intended or suitable for consumption or use by adults only.
    pub has_adult_consideration: Option<AdultOrientedEnumeration>,

    /// Defines the energy efficiency Category (also known as "class" or "rating") for a product according to an international energy efficiency standard.
    pub has_energy_consumption_details: Option<EnergyConsumptionDetails>,

    /// A product measurement, for example the inseam of pants, the wheel size of a bicycle, or the gauge of a screw. Usually an exact measurement, but can also be a range of measurements for adjustable products, for example belts and ski bindings.
    pub has_measurement: Option<QuantitativeValue>,

    /// Specifies a MerchantReturnPolicy that may be applicable.
    pub has_merchant_return_policy: Option<MerchantReturnPolicy>,

    /// The height of the item.
    pub height: Option<height>,

    /// Indicates the <a class="localLink" href="/productGroupID">productGroupID</a> for a <a class="localLink" href="/ProductGroup">ProductGroup</a> that this product <a class="localLink" href="/isVariantOf">isVariantOf</a>.
    pub in_product_group_with_id: Option<Text>,

    /// A pointer to another product (or multiple products) for which this product is an accessory or spare part.
    pub is_accessory_or_spare_part_for: Option<Product>,

    /// A pointer to another product (or multiple products) for which this product is a consumable.
    pub is_consumable_for: Option<Product>,

    /// A pointer to another, somehow related product (or multiple products).
    pub is_related_to: Option<isRelatedTo>,

    /// A pointer to another, functionally similar product (or multiple products).
    pub is_similar_to: Option<isSimilarTo>,

    /// Indicates the kind of product that this is a variant of. In the case of <a class="localLink" href="/ProductModel">ProductModel</a>, this is a pointer (from a ProductModel) to a base product from which this product is a variant. It is safe to infer that the variant inherits all product features from the base model, unless defined locally. This is not transitive. In the case of a <a class="localLink" href="/ProductGroup">ProductGroup</a>, the group description also serves as a template, representing a set of Products that vary on explicitly defined, specific dimensions only (so it defines both a set of variants, as well as which values distinguish amongst those variants). When used with <a class="localLink" href="/ProductGroup">ProductGroup</a>, this property can apply to any <a class="localLink" href="/Product">Product</a> included in the group.
    pub is_variant_of: Option<isVariantOf>,

    /// A predefined value from OfferItemCondition specifying the condition of the product or service, or the products or services included in the offer. Also used for product return policies to specify the condition of products accepted for returns.
    pub item_condition: Option<OfferItemCondition>,

    /// An associated logo.
    pub logo: Option<logo>,

    /// The manufacturer of the product.
    pub manufacturer: Option<Organization>,

    /// The <a class="localLink" href="/mobileUrl">mobileUrl</a> property is provided for specific situations in which data consumers need to determine whether one of several provided URLs is a dedicated 'mobile site'.<br/><br/>  To discourage over-use, and reflecting intial usecases, the property is expected only on <a class="localLink" href="/Product">Product</a> and <a class="localLink" href="/Offer">Offer</a>, rather than <a class="localLink" href="/Thing">Thing</a>. The general trend in web technology is towards <a href="https://en.wikipedia.org/wiki/Responsive_web_design">responsive design</a> in which content can be flexibly adapted to a wide range of browsing environments. Pages and sites referenced with the long-established <a class="localLink" href="/url">url</a> property should ideally also be usable on a wide variety of devices, including mobile phones. In most cases, it would be pointless and counter productive to attempt to update all <a class="localLink" href="/url">url</a> markup to use <a class="localLink" href="/mobileUrl">mobileUrl</a> for more mobile-oriented pages. The property is intended for the case when items (primarily <a class="localLink" href="/Product">Product</a> and <a class="localLink" href="/Offer">Offer</a>) have extra URLs hosted on an additional "mobile site" alongside the main one. It should not be taken as an endorsement of this publication style.
    pub mobile_url: Option<Text>,

    /// The model of the product. Use with the URL of a ProductModel or a textual representation of the model identifier. The URL of the ProductModel can be from an external source. It is recommended to additionally provide strong product identifiers via the gtin8/gtin13/gtin14 and mpn properties.
    pub model: Option<model>,

    /// The Manufacturer Part Number (MPN) of the product, or the product to which the offer refers.
    pub mpn: Option<Text>,

    /// Provides negative considerations regarding something, most typically in pro/con lists for reviews (alongside <a class="localLink" href="/positiveNotes">positiveNotes</a>). For symmetry <br/><br/>  In the case of a <a class="localLink" href="/Review">Review</a>, the property describes the <a class="localLink" href="/itemReviewed">itemReviewed</a> from the perspective of the review; in the case of a <a class="localLink" href="/Product">Product</a>, the product itself is being described. Since product descriptions  tend to emphasise positive claims, it may be relatively unusual to find <a class="localLink" href="/negativeNotes">negativeNotes</a> used in this way. Nevertheless for the sake of symmetry, <a class="localLink" href="/negativeNotes">negativeNotes</a> can be used on <a class="localLink" href="/Product">Product</a>.<br/><br/>  The property values can be expressed either as unstructured text (repeated as necessary), or if ordered, as a list (in which case the most negative is at the beginning of the list).
    pub negative_notes: Option<negativeNotes>,

    /// Indicates the <a href="https://en.wikipedia.org/wiki/NATO_Stock_Number">NATO stock number</a> (nsn) of a <a class="localLink" href="/Product">Product</a>.
    pub nsn: Option<Text>,

    /// Provides positive considerations regarding something, for example product highlights or (alongside <a class="localLink" href="/negativeNotes">negativeNotes</a>) pro/con lists for reviews.<br/><br/>  In the case of a <a class="localLink" href="/Review">Review</a>, the property describes the <a class="localLink" href="/itemReviewed">itemReviewed</a> from the perspective of the review; in the case of a <a class="localLink" href="/Product">Product</a>, the product itself is being described.<br/><br/>  The property values can be expressed either as unstructured text (repeated as necessary), or if ordered, as a list (in which case the most positive is at the beginning of the list).
    pub positive_notes: Option<positiveNotes>,

    /// The product identifier, such as ISBN. For example: <code>meta itemprop="productID" content="isbn:123-456-789"</code>.
    pub product_id: Option<Text>,

    /// The date of production of the item, e.g. vehicle.
    pub production_date: Option<Date>,

    /// The date the item, e.g. vehicle, was purchased by the current owner.
    pub purchase_date: Option<Date>,

    /// The release date of a product or product model. This can be used to distinguish the exact variant of a product.
    pub release_date: Option<Date>,

    /// The Stock Keeping Unit (SKU), i.e. a merchant-specific identifier for a product or service, or the product to which the offer refers.
    pub sku: Option<Text>,

    /// A slogan or motto associated with the item.
    pub slogan: Option<Text>,

    /// The weight of the product or person.
    pub weight: Option<QuantitativeValue>,

    /// The width of the item.
    pub width: Option<width>,

    /// This links to a node or nodes indicating the exact quantity of the products included in  an <a class="localLink" href="/Offer">Offer</a> or <a class="localLink" href="/ProductCollection">ProductCollection</a>.
    pub includes_object: Option<TypeAndQuantityNode>,
}

impl ProductCollection {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
