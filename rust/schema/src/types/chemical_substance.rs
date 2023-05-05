// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::bio_chem_entity::BioChemEntity;
use super::thing::Thing;
use super::chemical_role::chemicalRole;
use super::funding::funding;
use super::potential_use::potentialUse;

/// * MOD OF: https://pending.schema.org * COMMENT: A chemical substance is 'a portion of matter of constant composition, composed of molecular entities of the same type or of different types' (source: <a href="https://www.ebi.ac.uk/chebi/searchId.do?chebiId=59999">ChEBI:59999</a>). * EXTEND FROM: https://schema.org/BioChemEntity
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ChemicalSubstance {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<ChemicalSubstanceOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ChemicalSubstanceOptions {
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

    /// Disease associated to this BioChemEntity. Such disease can be a MedicalCondition or a URL. If you want to add an evidence supporting the association, please use PropertyValue.
    pub associated_disease: Option<BioChemEntity>,

    /// A BioChemEntity that is known to interact with this item.
    pub bio_chem_interaction: Option<BioChemEntity>,

    /// A similar BioChemEntity, e.g., obtained by fingerprint similarity algorithms.
    pub bio_chem_similarity: Option<BioChemEntity>,

    /// A role played by the BioChemEntity within a biological context.
    pub biological_role: Option<BioChemEntity>,

    /// A <a class="localLink" href="/Grant">Grant</a> that directly or indirectly provide funding or sponsorship for this item. See also <a class="localLink" href="/ownershipFundingInfo">ownershipFundingInfo</a>.
    pub funding: Option<funding>,

    /// Indicates a BioChemEntity that (in some sense) has this BioChemEntity as a part.
    pub has_bio_chem_entity_part: Option<BioChemEntity>,

    /// Molecular function performed by this BioChemEntity; please use PropertyValue if you want to include any evidence.
    pub has_molecular_function: Option<BioChemEntity>,

    /// A common representation such as a protein sequence or chemical structure for this entity. For images use schema.org/image.
    pub has_representation: Option<BioChemEntity>,

    /// Another BioChemEntity encoding by this one.
    pub is_encoded_by_bio_chem_entity: Option<BioChemEntity>,

    /// Biological process this BioChemEntity is involved in; please use PropertyValue if you want to include any evidence.
    pub is_involved_in_biological_process: Option<BioChemEntity>,

    /// Subcellular location where this BioChemEntity is located; please use PropertyValue if you want to include any evidence.
    pub is_located_in_subcellular_location: Option<BioChemEntity>,

    /// Indicates a BioChemEntity that is (in some sense) a part of this BioChemEntity.
    pub is_part_of_bio_chem_entity: Option<BioChemEntity>,

    /// The taxonomic grouping of the organism that expresses, encodes, or in some way related to the BioChemEntity.
    pub taxonomic_range: Option<BioChemEntity>,

    /// The chemical composition describes the identity and relative ratio of the chemical elements that make up the substance.
    pub chemical_composition: Option<ChemicalSubstance>,

    /// A role played by the BioChemEntity within a chemical context.
    pub chemical_role: Option<chemicalRole>,

    /// Intended use of the BioChemEntity by humans.
    pub potential_use: Option<potentialUse>,
}

impl ChemicalSubstance {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
