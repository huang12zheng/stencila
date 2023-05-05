// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::rating::Rating;
use super::thing::Thing;
use super::author::author;
use super::review_aspect::reviewAspect;

/// * COMMENT: An EndorsementRating is a rating that expresses some level of endorsement, for example inclusion in a "critic's pick" blog, a "Like" or "+1" on a social network. It can be considered the <a class="localLink" href="/result">result</a> of an <a class="localLink" href="/EndorseAction">EndorseAction</a> in which the <a class="localLink" href="/object">object</a> of the action is rated positively by some <a class="localLink" href="/agent">agent</a>. As is common elsewhere in schema.org, it is sometimes more useful to describe the results of such an action without explicitly describing the <a class="localLink" href="/Action">Action</a>.<br/><br/>  An <a class="localLink" href="/EndorsementRating">EndorsementRating</a> may be part of a numeric scale or organized system, but this is not required: having an explicit type for indicating a positive, endorsement rating is particularly useful in the absence of numeric scales as it helps consumers understand that the rating is broadly positive. * EXTEND FROM: https://schema.org/Rating
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct EndorsementRating {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<EndorsementRatingOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct EndorsementRatingOptions {
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

    /// The author of this content or rating. Please note that author is special in that HTML 5 provides a special mechanism for indicating authorship via the rel tag. That is equivalent to this and may be used interchangeably.
    pub author: Option<author>,

    /// The highest value allowed in this rating system. If bestRating is omitted, 5 is assumed.
    pub best_rating: Option<Rating>,

    /// A short explanation (e.g. one to two sentences) providing background context and other information that led to the conclusion expressed in the rating. This is particularly applicable to ratings associated with "fact check" markup using <a class="localLink" href="/ClaimReview">ClaimReview</a>.
    pub rating_explanation: Option<Rating>,

    /// The rating for the content.<br/><br/>  Usage guidelines:<br/><br/>  <ul> <li>Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similar Unicode symbols.</li> <li>Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.</li> </ul>
    pub rating_value: Option<Rating>,

    /// This Review or Rating is relevant to this part or facet of the itemReviewed.
    pub review_aspect: Option<reviewAspect>,

    /// The lowest value allowed in this rating system. If worstRating is omitted, 1 is assumed.
    pub worst_rating: Option<Rating>,
}

impl EndorsementRating {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
