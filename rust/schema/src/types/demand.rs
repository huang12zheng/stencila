// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::accepted_payment_method_prop_enum::AcceptedPaymentMethodPropEnum;
use super::action::Action;
use super::area_served_prop_enum::AreaServedPropEnum;
use super::asin_prop_enum::AsinPropEnum;
use super::availability_ends_prop_enum::AvailabilityEndsPropEnum;
use super::availability_starts_prop_enum::AvailabilityStartsPropEnum;
use super::business_entity_type::BusinessEntityType;
use super::business_function::BusinessFunction;
use super::delivery_method::DeliveryMethod;
use super::eligible_region_prop_enum::EligibleRegionPropEnum;
use super::gtin_prop_enum::GtinPropEnum;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::ineligible_region_prop_enum::IneligibleRegionPropEnum;
use super::item_availability::ItemAvailability;
use super::item_offered_prop_enum::ItemOfferedPropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::offer_item_condition::OfferItemCondition;
use super::place::Place;
use super::price_specification::PriceSpecification;
use super::quantitative_value::QuantitativeValue;
use super::seller_prop_enum::SellerPropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::type_and_quantity_node::TypeAndQuantityNode;
use super::url::URL;
use super::valid_from_prop_enum::ValidFromPropEnum;
use super::valid_through_prop_enum::ValidThroughPropEnum;
use super::warranty_promise::WarrantyPromise;

/// https://schema.org/Demand
/// * COMMENT:
/// A demand entity represents the public, not necessarily binding, not necessarily exclusive, announcement by an organization or person to seek a certain type of goods or services. For describing demand using this type, the very same properties used for Offer apply.
/// * EXTEND FROM:
/// https://schema.org/Intangible
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Demand {
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

    /// The payment method(s) accepted by seller for this offer.
    pub accepted_payment_method_prop_enum: Option<AcceptedPaymentMethodPropEnum>,

    /// The amount of time that is required between accepting the offer and the actual usage of the resource or service.
    pub advance_booking_requirement_prop_enum: Option<QuantitativeValue>,

    /// The geographic area where a service or offered item is provided.
    pub area_served_prop_enum: Option<AreaServedPropEnum>,

    /// An Amazon Standard Identification Number (ASIN) is a 10-character alphanumeric unique identifier assigned by Amazon.com and its partners for product identification within the Amazon organization (summary from <a href="https://en.wikipedia.org/wiki/Amazon_Standard_Identification_Number">Wikipedia</a>'s article).<br/><br/>  Note also that this is a definition for how to include ASINs in Schema.org data, and not a definition of ASINs in general - see documentation from Amazon for authoritative details. ASINs are most commonly encoded as text strings, but the [asin] property supports URL/URI as potential values too.
    pub asin_prop_enum: Option<AsinPropEnum>,

    /// The availability of this item&#x2014;for example In stock, Out of stock, Pre-order, etc.
    pub availability_prop_enum: Option<ItemAvailability>,

    /// The end of the availability of the product or service included in the offer.
    pub availability_ends_prop_enum: Option<AvailabilityEndsPropEnum>,

    /// The beginning of the availability of the product or service included in the offer.
    pub availability_starts_prop_enum: Option<AvailabilityStartsPropEnum>,

    /// The place(s) from which the offer can be obtained (e.g. store locations).
    pub available_at_or_from_prop_enum: Option<Place>,

    /// The delivery method(s) available for this offer.
    pub available_delivery_method_prop_enum: Option<DeliveryMethod>,

    /// The business function (e.g. sell, lease, repair, dispose) of the offer or component of a bundle (TypeAndQuantityNode). The default is http://purl.org/goodrelations/v1#Sell.
    pub business_function_prop_enum: Option<BusinessFunction>,

    /// The typical delay between the receipt of the order and the goods either leaving the warehouse or being prepared for pickup, in case the delivery method is on site pickup.
    pub delivery_lead_time_prop_enum: Option<QuantitativeValue>,

    /// The type(s) of customers for which the given offer is valid.
    pub eligible_customer_type_prop_enum: Option<BusinessEntityType>,

    /// The duration for which the given offer is valid.
    pub eligible_duration_prop_enum: Option<QuantitativeValue>,

    /// The interval and unit of measurement of ordering quantities for which the offer or price specification is valid. This allows e.g. specifying that a certain freight charge is valid only for a certain quantity.
    pub eligible_quantity_prop_enum: Option<QuantitativeValue>,

    /// The ISO 3166-1 (ISO 3166-1 alpha-2) or ISO 3166-2 code, the place, or the GeoShape for the geo-political region(s) for which the offer or delivery charge specification is valid.<br/><br/>  See also <a class="localLink" href="/ineligibleRegion">ineligibleRegion</a>.
    pub eligible_region_prop_enum: Option<EligibleRegionPropEnum>,

    /// The transaction volume, in a monetary unit, for which the offer or price specification is valid, e.g. for indicating a minimal purchasing volume, to express free shipping above a certain order volume, or to limit the acceptance of credit cards to purchases to a certain minimal amount.
    pub eligible_transaction_volume_prop_enum: Option<PriceSpecification>,

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

    /// This links to a node or nodes indicating the exact quantity of the products included in  an <a class="localLink" href="/Offer">Offer</a> or <a class="localLink" href="/ProductCollection">ProductCollection</a>.
    pub includes_object_prop_enum: Option<TypeAndQuantityNode>,

    /// The ISO 3166-1 (ISO 3166-1 alpha-2) or ISO 3166-2 code, the place, or the GeoShape for the geo-political region(s) for which the offer or delivery charge specification is not valid, e.g. a region where the transaction is not allowed.<br/><br/>  See also <a class="localLink" href="/eligibleRegion">eligibleRegion</a>.
    pub ineligible_region_prop_enum: Option<IneligibleRegionPropEnum>,

    /// The current approximate inventory level for the item or items.
    pub inventory_level_prop_enum: Option<QuantitativeValue>,

    /// A predefined value from OfferItemCondition specifying the condition of the product or service, or the products or services included in the offer. Also used for product return policies to specify the condition of products accepted for returns.
    pub item_condition_prop_enum: Option<OfferItemCondition>,

    /// An item being offered (or demanded). The transactional nature of the offer or demand is documented using <a class="localLink" href="/businessFunction">businessFunction</a>, e.g. sell, lease etc. While several common expected types are listed explicitly in this definition, others can be used. Using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
    pub item_offered_prop_enum: Option<ItemOfferedPropEnum>,

    /// The Manufacturer Part Number (MPN) of the product, or the product to which the offer refers.
    pub mpn_prop_enum: Option<Text>,

    /// One or more detailed price specifications, indicating the unit price and delivery or payment charges.
    pub price_specification_prop_enum: Option<PriceSpecification>,

    /// An entity which offers (sells / leases / lends / loans) the services / goods.  A seller may also be a provider.
    pub seller_prop_enum: Option<SellerPropEnum>,

    /// The serial number or any alphanumeric identifier of a particular product. When attached to an offer, it is a shortcut for the serial number of the product included in the offer.
    pub serial_number_prop_enum: Option<Text>,

    /// The Stock Keeping Unit (SKU), i.e. a merchant-specific identifier for a product or service, or the product to which the offer refers.
    pub sku_prop_enum: Option<Text>,

    /// The date when the item becomes valid.
    pub valid_from_prop_enum: Option<ValidFromPropEnum>,

    /// The date after when the item is not valid. For example the end of an offer, salary period, or a period of opening hours.
    pub valid_through_prop_enum: Option<ValidThroughPropEnum>,

    /// The warranty promise(s) included in the offer.
    pub warranty_prop_enum: Option<WarrantyPromise>,
}

impl Demand {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
