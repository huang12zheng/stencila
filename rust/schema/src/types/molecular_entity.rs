// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::bio_chem_entity::BioChemEntity;
use super::defined_term::DefinedTerm;
use super::gene::Gene;
use super::grant::Grant;
use super::text::Text;
use super::url::URL;
use super::associated_disease::associatedDisease;
use super::has_molecular_function::hasMolecularFunction;
use super::has_representation::hasRepresentation;
use super::identifier::identifier;
use super::image::image;
use super::is_involved_in_biological_process::isInvolvedInBiologicalProcess;
use super::is_located_in_subcellular_location::isLocatedInSubcellularLocation;
use super::main_entity_of_page::mainEntityOfPage;
use super::molecular_weight::molecularWeight;
use super::monoisotopic_molecular_weight::monoisotopicMolecularWeight;
use super::subject_of::subjectOf;
use super::taxonomic_range::taxonomicRange;

/// * MOD OF: https://pending.schema.org * COMMENT: Any constitutionally or isotopically distinct atom, molecule, ion, ion pair, radical, radical ion, complex, conformer etc., identifiable as a separately distinguishable entity. * EXTEND FROM: https://schema.org/BioChemEntity
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct MolecularEntity {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<MolecularEntityOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct MolecularEntityOptions {
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

    /// Disease associated to this BioChemEntity. Such disease can be a MedicalCondition or a URL. If you want to add an evidence supporting the association, please use PropertyValue.
    pub associated_disease: Option<associatedDisease>,

    /// A BioChemEntity that is known to interact with this item.
    pub bio_chem_interaction: Option<BioChemEntity>,

    /// A similar BioChemEntity, e.g., obtained by fingerprint similarity algorithms.
    pub bio_chem_similarity: Option<BioChemEntity>,

    /// A role played by the BioChemEntity within a biological context.
    pub biological_role: Option<DefinedTerm>,

    /// A <a class="localLink" href="/Grant">Grant</a> that directly or indirectly provide funding or sponsorship for this item. See also <a class="localLink" href="/ownershipFundingInfo">ownershipFundingInfo</a>.
    pub funding: Option<Grant>,

    /// Indicates a BioChemEntity that (in some sense) has this BioChemEntity as a part.
    pub has_bio_chem_entity_part: Option<BioChemEntity>,

    /// Molecular function performed by this BioChemEntity; please use PropertyValue if you want to include any evidence.
    pub has_molecular_function: Option<hasMolecularFunction>,

    /// A common representation such as a protein sequence or chemical structure for this entity. For images use schema.org/image.
    pub has_representation: Option<hasRepresentation>,

    /// Another BioChemEntity encoding by this one.
    pub is_encoded_by_bio_chem_entity: Option<Gene>,

    /// Biological process this BioChemEntity is involved in; please use PropertyValue if you want to include any evidence.
    pub is_involved_in_biological_process: Option<isInvolvedInBiologicalProcess>,

    /// Subcellular location where this BioChemEntity is located; please use PropertyValue if you want to include any evidence.
    pub is_located_in_subcellular_location: Option<isLocatedInSubcellularLocation>,

    /// Indicates a BioChemEntity that is (in some sense) a part of this BioChemEntity.
    pub is_part_of_bio_chem_entity: Option<BioChemEntity>,

    /// The taxonomic grouping of the organism that expresses, encodes, or in some way related to the BioChemEntity.
    pub taxonomic_range: Option<taxonomicRange>,

    /// A role played by the BioChemEntity within a chemical context.
    pub chemical_role: Option<DefinedTerm>,

    /// Non-proprietary identifier for molecular entity that can be used in printed and electronic data sources thus enabling easier linking of diverse data compilations.
    pub in_ch_i: Option<Text>,

    /// InChIKey is a hashed version of the full InChI (using the SHA-256 algorithm).
    pub in_ch_i_key: Option<Text>,

    /// Systematic method of naming chemical compounds as recommended by the International Union of Pure and Applied Chemistry (IUPAC).
    pub iupac_name: Option<Text>,

    /// The empirical formula is the simplest whole number ratio of all the atoms in a molecule.
    pub molecular_formula: Option<Text>,

    /// This is the molecular weight of the entity being described, not of the parent. Units should be included in the form '&lt;Number&gt; &lt;unit&gt;', for example '12 amu' or as '&lt;QuantitativeValue&gt;.
    pub molecular_weight: Option<molecularWeight>,

    /// The monoisotopic mass is the sum of the masses of the atoms in a molecule using the unbound, ground-state, rest mass of the principal (most abundant) isotope for each element instead of the isotopic average mass. Please include the units in the form '&lt;Number&gt; &lt;unit&gt;', for example '770.230488 g/mol' or as '&lt;QuantitativeValue&gt;.
    pub monoisotopic_molecular_weight: Option<monoisotopicMolecularWeight>,

    /// Intended use of the BioChemEntity by humans.
    pub potential_use: Option<DefinedTerm>,

    /// A specification in form of a line notation for describing the structure of chemical species using short ASCII strings.  Double bond stereochemistry \ indicators may need to be escaped in the string in formats where the backslash is an escape character.
    pub smiles: Option<Text>,
}

impl MolecularEntity {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
