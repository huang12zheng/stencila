// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::thing::Thing;
use super::additional_property::additionalProperty;
use super::item_condition::itemCondition;
use super::merchant_return_days::merchantReturnDays;
use super::return_policy_category::returnPolicyCategory;

/// * MOD OF: https://pending.schema.org * COMMENT: A MerchantReturnPolicy provides information about product return policies associated with an <a class="localLink" href="/Organization">Organization</a>, <a class="localLink" href="/Product">Product</a>, or <a class="localLink" href="/Offer">Offer</a>. * EXTEND FROM: https://schema.org/Intangible
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct MerchantReturnPolicy {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<MerchantReturnPolicyOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct MerchantReturnPolicyOptions {
    /// An additional type for the item, typically used for adding more specific types from external vocabularies in microdata syntax. This is a relationship between something and a class that the thing is in. In RDFa syntax, it is better to use the native RDFa syntax - the 'typeof' attribute - for multiple types. Schema.org tools may have only weaker understanding of extra types, in particular those defined externally.
    pub additional_type: Option<Thing>,

    /// An alias for the item.
    pub alternate_name: Option<Thing>,

    /// A description of the item.
    pub description: Option<Thing>,

    /// A sub property of description. A short description of the item used to disambiguate from other, similar items. Information from other properties (in particular, name) may be necessary for the description to be useful for disambiguation.
    pub disambiguating_description: Option<Thing>,

    /// The identifier property represents any kind of identifier for any kind of <a class="localLink" href="/Thing">Thing</a>, such as ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing many of these, either as textual strings or as URL (URI) links. See <a href="/docs/datamodel.html#identifierBg">background notes</a> for more details.
    pub identifier: Option<Thing>,

    /// An image of the item. This can be a <a class="localLink" href="/URL">URL</a> or a fully described <a class="localLink" href="/ImageObject">ImageObject</a>.
    pub image: Option<Thing>,

    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being described. See <a href="/docs/datamodel.html#mainEntityBackground">background notes</a> for details.
    pub main_entity_of_page: Option<Thing>,

    /// The name of the item.
    pub name: Option<Thing>,

    /// Indicates a potential Action, which describes an idealized action in which this thing would play an 'object' role.
    pub potential_action: Option<Thing>,

    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL of the item's Wikipedia page, Wikidata entry, or official website.
    pub same_as: Option<Thing>,

    /// A CreativeWork or Event about this Thing.
    pub subject_of: Option<Thing>,

    /// URL of the item.
    pub url: Option<Thing>,

    /// A property-value pair representing an additional characteristic of the entity, e.g. a product feature or another characteristic for which there is no matching property in schema.org.<br/><br/>  Note: Publishers should be aware that applications designed to use specific schema.org properties (e.g. http://schema.org/width, http://schema.org/color, http://schema.org/gtin13, ...) will typically expect such data to be provided using those properties, rather than using the generic property/value mechanism.
    pub additional_property: Option<additionalProperty>,

    /// A country where a particular merchant return policy applies to, for example the two-letter ISO 3166-1 alpha-2 country code.
    pub applicable_country: Option<MerchantReturnPolicy>,

    /// The type of return fees if the product is returned due to customer remorse.
    pub customer_remorse_return_fees: Option<MerchantReturnPolicy>,

    /// The method (from an enumeration) by which the customer obtains a return shipping label for a product returned due to customer remorse.
    pub customer_remorse_return_label_source: Option<MerchantReturnPolicy>,

    /// The amount of shipping costs if a product is returned due to customer remorse. Applicable when property <a class="localLink" href="/customerRemorseReturnFees">customerRemorseReturnFees</a> equals <a class="localLink" href="/ReturnShippingFees">ReturnShippingFees</a>.
    pub customer_remorse_return_shipping_fees_amount: Option<MerchantReturnPolicy>,

    /// Are in-store returns offered? (For more advanced return methods use the <a class="localLink" href="/returnMethod">returnMethod</a> property.)
    pub in_store_returns_offered: Option<MerchantReturnPolicy>,

    /// A predefined value from OfferItemCondition specifying the condition of the product or service, or the products or services included in the offer. Also used for product return policies to specify the condition of products accepted for returns.
    pub item_condition: Option<itemCondition>,

    /// The type of return fees for returns of defect products.
    pub item_defect_return_fees: Option<MerchantReturnPolicy>,

    /// The method (from an enumeration) by which the customer obtains a return shipping label for a defect product.
    pub item_defect_return_label_source: Option<MerchantReturnPolicy>,

    /// Amount of shipping costs for defect product returns. Applicable when property <a class="localLink" href="/itemDefectReturnFees">itemDefectReturnFees</a> equals <a class="localLink" href="/ReturnShippingFees">ReturnShippingFees</a>.
    pub item_defect_return_shipping_fees_amount: Option<MerchantReturnPolicy>,

    /// Specifies either a fixed return date or the number of days (from the delivery date) that a product can be returned. Used when the <a class="localLink" href="/returnPolicyCategory">returnPolicyCategory</a> property is specified as <a class="localLink" href="/MerchantReturnFiniteReturnWindow">MerchantReturnFiniteReturnWindow</a>.
    pub merchant_return_days: Option<merchantReturnDays>,

    /// Specifies a Web page or service by URL, for product returns.
    pub merchant_return_link: Option<MerchantReturnPolicy>,

    /// A refund type, from an enumerated list.
    pub refund_type: Option<MerchantReturnPolicy>,

    /// Use <a class="localLink" href="/MonetaryAmount">MonetaryAmount</a> to specify a fixed restocking fee for product returns, or use <a class="localLink" href="/Number">Number</a> to specify a percentage of the product price paid by the customer.
    pub restocking_fee: Option<MerchantReturnPolicy>,

    /// The type of return fees for purchased products (for any return reason).
    pub return_fees: Option<MerchantReturnPolicy>,

    /// The method (from an enumeration) by which the customer obtains a return shipping label for a product returned for any reason.
    pub return_label_source: Option<MerchantReturnPolicy>,

    /// The type of return method offered, specified from an enumeration.
    pub return_method: Option<MerchantReturnPolicy>,

    /// Specifies an applicable return policy (from an enumeration).
    pub return_policy_category: Option<returnPolicyCategory>,

    /// The country where the product has to be sent to for returns, for example "Ireland" using the <a class="localLink" href="/name">name</a> property of <a class="localLink" href="/Country">Country</a>. You can also provide the two-letter <a href="http://en.wikipedia.org/wiki/ISO_3166-1">ISO 3166-1 alpha-2 country code</a>. Note that this can be different from the country where the product was originally shipped from or sent to.
    pub return_policy_country: Option<MerchantReturnPolicy>,

    /// Seasonal override of a return policy.
    pub return_policy_seasonal_override: Option<MerchantReturnPolicy>,

    /// Amount of shipping costs for product returns (for any reason). Applicable when property <a class="localLink" href="/returnFees">returnFees</a> equals <a class="localLink" href="/ReturnShippingFees">ReturnShippingFees</a>.
    pub return_shipping_fees_amount: Option<MerchantReturnPolicy>,
}

impl MerchantReturnPolicy {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
