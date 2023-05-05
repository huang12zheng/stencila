// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::adult_oriented_enumeration::AdultOrientedEnumeration;
use super::aggregate_rating::AggregateRating;
use super::asin_prop_enum::AsinPropEnum;
use super::audience::Audience;
use super::boolean::Boolean;
use super::brand_prop_enum::BrandPropEnum;
use super::category_prop_enum::CategoryPropEnum;
use super::country::Country;
use super::date::Date;
use super::depth_prop_enum::DepthPropEnum;
use super::energy_consumption_details::EnergyConsumptionDetails;
use super::grant::Grant;
use super::gtin_prop_enum::GtinPropEnum;
use super::height_prop_enum::HeightPropEnum;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::is_related_to_prop_enum::IsRelatedToPropEnum;
use super::is_similar_to_prop_enum::IsSimilarToPropEnum;
use super::is_variant_of_prop_enum::IsVariantOfPropEnum;
use super::keywords_prop_enum::KeywordsPropEnum;
use super::legal_status_prop_enum::LegalStatusPropEnum;
use super::logo_prop_enum::LogoPropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::material_prop_enum::MaterialPropEnum;
use super::maximum_dose_schedule::MaximumDoseSchedule;
use super::medical_code::MedicalCode;
use super::medical_guideline::MedicalGuideline;
use super::medical_specialty::MedicalSpecialty;
use super::medical_study::MedicalStudy;
use super::medicine_system::MedicineSystem;
use super::merchant_return_policy::MerchantReturnPolicy;
use super::model_prop_enum::ModelPropEnum;
use super::negative_notes_prop_enum::NegativeNotesPropEnum;
use super::offer_item_condition::OfferItemCondition;
use super::offers_prop_enum::OffersPropEnum;
use super::organization::Organization;
use super::pattern_prop_enum::PatternPropEnum;
use super::positive_notes_prop_enum::PositiveNotesPropEnum;
use super::product::Product;
use super::property_value::PropertyValue;
use super::quantitative_value::QuantitativeValue;
use super::recommended_dose_schedule::RecommendedDoseSchedule;
use super::review::Review;
use super::size_prop_enum::SizePropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;
use super::width_prop_enum::WidthPropEnum;

/// https://schema.org/DietarySupplement
/// * COMMENT:
/// A product taken by mouth that contains a dietary ingredient intended to supplement the diet. Dietary ingredients may include vitamins, minerals, herbs or other botanicals, amino acids, and substances such as enzymes, organ tissues, glandulars and metabolites.
/// * EXTEND FROM:
/// https://schema.org/Product, https://schema.org/Substance
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct DietarySupplement {
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

    /// A property-value pair representing an additional characteristic of the entity, e.g. a product feature or another characteristic for which there is no matching property in schema.org.<br/><br/>  Note: Publishers should be aware that applications designed to use specific schema.org properties (e.g. http://schema.org/width, http://schema.org/color, http://schema.org/gtin13, ...) will typically expect such data to be provided using those properties, rather than using the generic property/value mechanism.
    pub additional_property_prop_enum: Option<PropertyValue>,

    /// The overall rating, based on a collection of reviews or ratings, of the item.
    pub aggregate_rating_prop_enum: Option<AggregateRating>,

    /// An Amazon Standard Identification Number (ASIN) is a 10-character alphanumeric unique identifier assigned by Amazon.com and its partners for product identification within the Amazon organization (summary from <a href="https://en.wikipedia.org/wiki/Amazon_Standard_Identification_Number">Wikipedia</a>'s article).<br/><br/>  Note also that this is a definition for how to include ASINs in Schema.org data, and not a definition of ASINs in general - see documentation from Amazon for authoritative details. ASINs are most commonly encoded as text strings, but the [asin] property supports URL/URI as potential values too.
    pub asin_prop_enum: Option<AsinPropEnum>,

    /// An intended audience, i.e. a group for whom something was created.
    pub audience_prop_enum: Option<Audience>,

    /// An award won by or for this item.
    pub award_prop_enum: Option<Text>,

    /// The brand(s) associated with a product or service, or the brand(s) maintained by an organization or business person.
    pub brand_prop_enum: Option<BrandPropEnum>,

    /// A category for the item. Greater signs or slashes can be used to informally indicate a category hierarchy.
    pub category_prop_enum: Option<CategoryPropEnum>,

    /// The color of the product.
    pub color_prop_enum: Option<Text>,

    /// The place where the product was assembled.
    pub country_of_assembly_prop_enum: Option<Text>,

    /// The place where the item (typically <a class="localLink" href="/Product">Product</a>) was last processed and tested before importation.
    pub country_of_last_processing_prop_enum: Option<Text>,

    /// The country of origin of something, including products as well as creative  works such as movie and TV content.<br/><br/>  In the case of TV and movie, this would be the country of the principle offices of the production company or individual responsible for the movie. For other kinds of <a class="localLink" href="/CreativeWork">CreativeWork</a> it is difficult to provide fully general guidance, and properties such as <a class="localLink" href="/contentLocation">contentLocation</a> and <a class="localLink" href="/locationCreated">locationCreated</a> may be more applicable.<br/><br/>  In the case of products, the country of origin of the product. The exact interpretation of this may vary by context and product type, and cannot be fully enumerated here.
    pub country_of_origin_prop_enum: Option<Country>,

    /// The depth of the item.
    pub depth_prop_enum: Option<DepthPropEnum>,

    /// A <a class="localLink" href="/Grant">Grant</a> that directly or indirectly provide funding or sponsorship for this item. See also <a class="localLink" href="/ownershipFundingInfo">ownershipFundingInfo</a>.
    pub funding_prop_enum: Option<Grant>,

    /// A Global Trade Item Number (<a href="https://www.gs1.org/standards/id-keys/gtin">GTIN</a>). GTINs identify trade items, including products and services, using numeric identification codes.<br/><br/>  The GS1 <a href="https://www.gs1.org/standards/Digital-Link/">digital link specifications</a> express GTINs as URLs (URIs, IRIs, etc.). Details including regular expression examples can be found in, Section 6 of the GS1 URI Syntax specification; see also <a href="https://github.com/schemaorg/schemaorg/issues/3156#issuecomment-1209522809">schema.org tracking issue</a> for schema.org-specific discussion. A correct <a class="localLink" href="/gtin">gtin</a> value should be a valid GTIN, which means that it should be an all-numeric string of either 8, 12, 13 or 14 digits, or a "GS1 Digital Link" URL based on such a string. The numeric component should also have a <a href="https://www.gs1.org/services/check-digit-calculator">valid GS1 check digit</a> and meet the other rules for valid GTINs. See also <a href="http://www.gs1.org/barcodes/technical/idkeys/gtin">GS1's GTIN Summary</a> and <a href="https://en.wikipedia.org/wiki/Global_Trade_Item_Number">Wikipedia</a> for more details. Left-padding of the gtin values is not required or encouraged. The <a class="localLink" href="/gtin">gtin</a> property generalizes the earlier <a class="localLink" href="/gtin8">gtin8</a>, <a class="localLink" href="/gtin12">gtin12</a>, <a class="localLink" href="/gtin13">gtin13</a>, and <a class="localLink" href="/gtin14">gtin14</a> properties.<br/><br/>  Note also that this is a definition for how to include GTINs in Schema.org data, and not a definition of GTINs in general - see the GS1 documentation for authoritative details.
    pub gtin_prop_enum: Option<GtinPropEnum>,

    /// The GTIN-12 code of the product, or the product to which the offer refers. The GTIN-12 is the 12-digit GS1 Identification Key composed of a U.P.C. Company Prefix, Item Reference, and Check Digit used to identify trade items. See <a href="http://www.gs1.org/barcodes/technical/idkeys/gtin">GS1 GTIN Summary</a> for more details.
    pub gtin_12_prop_enum: Option<Text>,

    /// The GTIN-13 code of the product, or the product to which the offer refers. This is equivalent to 13-digit ISBN codes and EAN UCC-13. Former 12-digit UPC codes can be converted into a GTIN-13 code by simply adding a preceding zero. See <a href="http://www.gs1.org/barcodes/technical/idkeys/gtin">GS1 GTIN Summary</a> for more details.
    pub gtin_13_prop_enum: Option<Text>,

    /// The GTIN-14 code of the product, or the product to which the offer refers. See <a href="http://www.gs1.org/barcodes/technical/idkeys/gtin">GS1 GTIN Summary</a> for more details.
    pub gtin_14_prop_enum: Option<Text>,

    /// The GTIN-8 code of the product, or the product to which the offer refers. This code is also known as EAN/UCC-8 or 8-digit EAN. See <a href="http://www.gs1.org/barcodes/technical/idkeys/gtin">GS1 GTIN Summary</a> for more details.
    pub gtin_8_prop_enum: Option<Text>,

    /// Used to tag an item to be intended or suitable for consumption or use by adults only.
    pub has_adult_consideration_prop_enum: Option<AdultOrientedEnumeration>,

    /// Defines the energy efficiency Category (also known as "class" or "rating") for a product according to an international energy efficiency standard.
    pub has_energy_consumption_details_prop_enum: Option<EnergyConsumptionDetails>,

    /// A product measurement, for example the inseam of pants, the wheel size of a bicycle, or the gauge of a screw. Usually an exact measurement, but can also be a range of measurements for adjustable products, for example belts and ski bindings.
    pub has_measurement_prop_enum: Option<QuantitativeValue>,

    /// Specifies a MerchantReturnPolicy that may be applicable.
    pub has_merchant_return_policy_prop_enum: Option<MerchantReturnPolicy>,

    /// The height of the item.
    pub height_prop_enum: Option<HeightPropEnum>,

    /// Indicates the <a class="localLink" href="/productGroupID">productGroupID</a> for a <a class="localLink" href="/ProductGroup">ProductGroup</a> that this product <a class="localLink" href="/isVariantOf">isVariantOf</a>.
    pub in_product_group_with_id_prop_enum: Option<Text>,

    /// A pointer to another product (or multiple products) for which this product is an accessory or spare part.
    pub is_accessory_or_spare_part_for_prop_enum: Option<Product>,

    /// A pointer to another product (or multiple products) for which this product is a consumable.
    pub is_consumable_for_prop_enum: Option<Product>,

    /// Indicates whether this content is family friendly.
    pub is_family_friendly_prop_enum: Option<Boolean>,

    /// A pointer to another, somehow related product (or multiple products).
    pub is_related_to_prop_enum: Option<IsRelatedToPropEnum>,

    /// A pointer to another, functionally similar product (or multiple products).
    pub is_similar_to_prop_enum: Option<IsSimilarToPropEnum>,

    /// Indicates the kind of product that this is a variant of. In the case of <a class="localLink" href="/ProductModel">ProductModel</a>, this is a pointer (from a ProductModel) to a base product from which this product is a variant. It is safe to infer that the variant inherits all product features from the base model, unless defined locally. This is not transitive. In the case of a <a class="localLink" href="/ProductGroup">ProductGroup</a>, the group description also serves as a template, representing a set of Products that vary on explicitly defined, specific dimensions only (so it defines both a set of variants, as well as which values distinguish amongst those variants). When used with <a class="localLink" href="/ProductGroup">ProductGroup</a>, this property can apply to any <a class="localLink" href="/Product">Product</a> included in the group.
    pub is_variant_of_prop_enum: Option<IsVariantOfPropEnum>,

    /// A predefined value from OfferItemCondition specifying the condition of the product or service, or the products or services included in the offer. Also used for product return policies to specify the condition of products accepted for returns.
    pub item_condition_prop_enum: Option<OfferItemCondition>,

    /// Keywords or tags used to describe some item. Multiple textual entries in a keywords list are typically delimited by commas, or by repeating the property.
    pub keywords_prop_enum: Option<KeywordsPropEnum>,

    /// An associated logo.
    pub logo_prop_enum: Option<LogoPropEnum>,

    /// The manufacturer of the product.
    pub manufacturer_prop_enum: Option<Organization>,

    /// A material that something is made from, e.g. leather, wool, cotton, paper.
    pub material_prop_enum: Option<MaterialPropEnum>,

    /// The <a class="localLink" href="/mobileUrl">mobileUrl</a> property is provided for specific situations in which data consumers need to determine whether one of several provided URLs is a dedicated 'mobile site'.<br/><br/>  To discourage over-use, and reflecting intial usecases, the property is expected only on <a class="localLink" href="/Product">Product</a> and <a class="localLink" href="/Offer">Offer</a>, rather than <a class="localLink" href="/Thing">Thing</a>. The general trend in web technology is towards <a href="https://en.wikipedia.org/wiki/Responsive_web_design">responsive design</a> in which content can be flexibly adapted to a wide range of browsing environments. Pages and sites referenced with the long-established <a class="localLink" href="/url">url</a> property should ideally also be usable on a wide variety of devices, including mobile phones. In most cases, it would be pointless and counter productive to attempt to update all <a class="localLink" href="/url">url</a> markup to use <a class="localLink" href="/mobileUrl">mobileUrl</a> for more mobile-oriented pages. The property is intended for the case when items (primarily <a class="localLink" href="/Product">Product</a> and <a class="localLink" href="/Offer">Offer</a>) have extra URLs hosted on an additional "mobile site" alongside the main one. It should not be taken as an endorsement of this publication style.
    pub mobile_url_prop_enum: Option<Text>,

    /// The model of the product. Use with the URL of a ProductModel or a textual representation of the model identifier. The URL of the ProductModel can be from an external source. It is recommended to additionally provide strong product identifiers via the gtin8/gtin13/gtin14 and mpn properties.
    pub model_prop_enum: Option<ModelPropEnum>,

    /// The Manufacturer Part Number (MPN) of the product, or the product to which the offer refers.
    pub mpn_prop_enum: Option<Text>,

    /// Provides negative considerations regarding something, most typically in pro/con lists for reviews (alongside <a class="localLink" href="/positiveNotes">positiveNotes</a>). For symmetry <br/><br/>  In the case of a <a class="localLink" href="/Review">Review</a>, the property describes the <a class="localLink" href="/itemReviewed">itemReviewed</a> from the perspective of the review; in the case of a <a class="localLink" href="/Product">Product</a>, the product itself is being described. Since product descriptions  tend to emphasise positive claims, it may be relatively unusual to find <a class="localLink" href="/negativeNotes">negativeNotes</a> used in this way. Nevertheless for the sake of symmetry, <a class="localLink" href="/negativeNotes">negativeNotes</a> can be used on <a class="localLink" href="/Product">Product</a>.<br/><br/>  The property values can be expressed either as unstructured text (repeated as necessary), or if ordered, as a list (in which case the most negative is at the beginning of the list).
    pub negative_notes_prop_enum: Option<NegativeNotesPropEnum>,

    /// Indicates the <a href="https://en.wikipedia.org/wiki/NATO_Stock_Number">NATO stock number</a> (nsn) of a <a class="localLink" href="/Product">Product</a>.
    pub nsn_prop_enum: Option<Text>,

    /// An offer to provide this item&#x2014;for example, an offer to sell a product, rent the DVD of a movie, perform a service, or give away tickets to an event. Use <a class="localLink" href="/businessFunction">businessFunction</a> to indicate the kind of transaction offered, i.e. sell, lease, etc. This property can also be used to describe a <a class="localLink" href="/Demand">Demand</a>. While this property is listed as expected on a number of common types, it can be used in others. In that case, using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
    pub offers_prop_enum: Option<OffersPropEnum>,

    /// A pattern that something has, for example 'polka dot', 'striped', 'Canadian flag'. Values are typically expressed as text, although links to controlled value schemes are also supported.
    pub pattern_prop_enum: Option<PatternPropEnum>,

    /// Provides positive considerations regarding something, for example product highlights or (alongside <a class="localLink" href="/negativeNotes">negativeNotes</a>) pro/con lists for reviews.<br/><br/>  In the case of a <a class="localLink" href="/Review">Review</a>, the property describes the <a class="localLink" href="/itemReviewed">itemReviewed</a> from the perspective of the review; in the case of a <a class="localLink" href="/Product">Product</a>, the product itself is being described.<br/><br/>  The property values can be expressed either as unstructured text (repeated as necessary), or if ordered, as a list (in which case the most positive is at the beginning of the list).
    pub positive_notes_prop_enum: Option<PositiveNotesPropEnum>,

    /// The product identifier, such as ISBN. For example: <code>meta itemprop="productID" content="isbn:123-456-789"</code>.
    pub product_id_prop_enum: Option<Text>,

    /// The date of production of the item, e.g. vehicle.
    pub production_date_prop_enum: Option<Date>,

    /// The date the item, e.g. vehicle, was purchased by the current owner.
    pub purchase_date_prop_enum: Option<Date>,

    /// The release date of a product or product model. This can be used to distinguish the exact variant of a product.
    pub release_date_prop_enum: Option<Date>,

    /// A review of the item.
    pub review_prop_enum: Option<Review>,

    /// A standardized size of a product or creative work, specified either through a simple textual string (for example 'XL', '32Wx34L'), a  QuantitativeValue with a unitCode, or a comprehensive and structured <a class="localLink" href="/SizeSpecification">SizeSpecification</a>; in other cases, the <a class="localLink" href="/width">width</a>, <a class="localLink" href="/height">height</a>, <a class="localLink" href="/depth">depth</a> and <a class="localLink" href="/weight">weight</a> properties may be more applicable.
    pub size_prop_enum: Option<SizePropEnum>,

    /// The Stock Keeping Unit (SKU), i.e. a merchant-specific identifier for a product or service, or the product to which the offer refers.
    pub sku_prop_enum: Option<Text>,

    /// A slogan or motto associated with the item.
    pub slogan_prop_enum: Option<Text>,

    /// The weight of the product or person.
    pub weight_prop_enum: Option<QuantitativeValue>,

    /// The width of the item.
    pub width_prop_enum: Option<WidthPropEnum>,

    /// A medical code for the entity, taken from a controlled vocabulary or ontology such as ICD-9, DiseasesDB, MeSH, SNOMED-CT, RxNorm, etc.
    pub code_prop_enum: Option<MedicalCode>,

    /// A medical guideline related to this entity.
    pub guideline_prop_enum: Option<MedicalGuideline>,

    /// The drug or supplement's legal status, including any controlled substance schedules that apply.
    pub legal_status_prop_enum: Option<LegalStatusPropEnum>,

    /// The system of medicine that includes this MedicalEntity, for example 'evidence-based', 'homeopathic', 'chiropractic', etc.
    pub medicine_system_prop_enum: Option<MedicineSystem>,

    /// If applicable, the organization that officially recognizes this entity as part of its endorsed system of medicine.
    pub recognizing_authority_prop_enum: Option<Organization>,

    /// If applicable, a medical specialty in which this entity is relevant.
    pub relevant_specialty_prop_enum: Option<MedicalSpecialty>,

    /// A medical study or trial related to this entity.
    pub study_prop_enum: Option<MedicalStudy>,

    /// An active ingredient, typically chemical compounds and/or biologic substances.
    pub active_ingredient_prop_enum: Option<Text>,

    /// Recommended intake of this supplement for a given population as defined by a specific recommending authority.
    pub maximum_intake_prop_enum: Option<MaximumDoseSchedule>,

    /// True if this item's name is a proprietary/brand name (vs. generic name).
    pub is_proprietary_prop_enum: Option<Boolean>,

    /// The specific biochemical interaction through which this drug or supplement produces its pharmacological effect.
    pub mechanism_of_action_prop_enum: Option<Text>,

    /// The generic name of this drug or supplement.
    pub non_proprietary_name_prop_enum: Option<Text>,

    /// Proprietary name given to the diet plan, typically by its originator or creator.
    pub proprietary_name_prop_enum: Option<Text>,

    /// Recommended intake of this supplement for a given population as defined by a specific recommending authority.
    pub recommended_intake_prop_enum: Option<RecommendedDoseSchedule>,

    /// Any potential safety concern associated with the supplement. May include interactions with other drugs and foods, pregnancy, breastfeeding, known adverse reactions, and documented efficacy of the supplement.
    pub safety_consideration_prop_enum: Option<Text>,

    /// Characteristics of the population for which this is intended, or which typically uses it, e.g. 'adults'.
    pub target_population_prop_enum: Option<Text>,
}

impl DietarySupplement {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
