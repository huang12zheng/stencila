// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::associated_disease_prop_enum::AssociatedDiseasePropEnum;
use super::bio_chem_entity::BioChemEntity;
use super::defined_term::DefinedTerm;
use super::gene::Gene;
use super::grant::Grant;
use super::has_molecular_function_prop_enum::HasMolecularFunctionPropEnum;
use super::has_representation_prop_enum::HasRepresentationPropEnum;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::is_involved_in_biological_process_prop_enum::IsInvolvedInBiologicalProcessPropEnum;
use super::is_located_in_subcellular_location_prop_enum::IsLocatedInSubcellularLocationPropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::taxonomic_range_prop_enum::TaxonomicRangePropEnum;
use super::text::Text;
use super::url::URL;

/// https://schema.org/ChemicalSubstance
/// * MOD OF:
/// https://pending.schema.org
/// * COMMENT:
/// A chemical substance is 'a portion of matter of constant composition, composed of molecular entities of the same type or of different types' (source: <a href="https://www.ebi.ac.uk/chebi/searchId.do?chebiId=59999">ChEBI:59999</a>).
/// * EXTEND FROM:
/// https://schema.org/BioChemEntity
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct ChemicalSubstance {
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

    /// Disease associated to this BioChemEntity. Such disease can be a MedicalCondition or a URL. If you want to add an evidence supporting the association, please use PropertyValue.
    pub associated_disease_prop_enum: Option<AssociatedDiseasePropEnum>,

    /// A BioChemEntity that is known to interact with this item.
    pub bio_chem_interaction_prop_enum: Option<BioChemEntity>,

    /// A similar BioChemEntity, e.g., obtained by fingerprint similarity algorithms.
    pub bio_chem_similarity_prop_enum: Option<BioChemEntity>,

    /// A role played by the BioChemEntity within a biological context.
    pub biological_role_prop_enum: Option<DefinedTerm>,

    /// A <a class="localLink" href="/Grant">Grant</a> that directly or indirectly provide funding or sponsorship for this item. See also <a class="localLink" href="/ownershipFundingInfo">ownershipFundingInfo</a>.
    pub funding_prop_enum: Option<Grant>,

    /// Indicates a BioChemEntity that (in some sense) has this BioChemEntity as a part.
    pub has_bio_chem_entity_part_prop_enum: Option<BioChemEntity>,

    /// Molecular function performed by this BioChemEntity; please use PropertyValue if you want to include any evidence.
    pub has_molecular_function_prop_enum: Option<HasMolecularFunctionPropEnum>,

    /// A common representation such as a protein sequence or chemical structure for this entity. For images use schema.org/image.
    pub has_representation_prop_enum: Option<HasRepresentationPropEnum>,

    /// Another BioChemEntity encoding by this one.
    pub is_encoded_by_bio_chem_entity_prop_enum: Option<Gene>,

    /// Biological process this BioChemEntity is involved in; please use PropertyValue if you want to include any evidence.
    pub is_involved_in_biological_process_prop_enum: Option<IsInvolvedInBiologicalProcessPropEnum>,

    /// Subcellular location where this BioChemEntity is located; please use PropertyValue if you want to include any evidence.
    pub is_located_in_subcellular_location_prop_enum: Option<IsLocatedInSubcellularLocationPropEnum>,

    /// Indicates a BioChemEntity that is (in some sense) a part of this BioChemEntity.
    pub is_part_of_bio_chem_entity_prop_enum: Option<BioChemEntity>,

    /// The taxonomic grouping of the organism that expresses, encodes, or in some way related to the BioChemEntity.
    pub taxonomic_range_prop_enum: Option<TaxonomicRangePropEnum>,

    /// The chemical composition describes the identity and relative ratio of the chemical elements that make up the substance.
    pub chemical_composition_prop_enum: Option<Text>,

    /// A role played by the BioChemEntity within a chemical context.
    pub chemical_role_prop_enum: Option<DefinedTerm>,

    /// Intended use of the BioChemEntity by humans.
    pub potential_use_prop_enum: Option<DefinedTerm>,
}

impl ChemicalSubstance {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
