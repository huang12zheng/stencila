// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::number::Number;
use super::text::Text;
use super::url::URL;
use super::cvd_collection_date::cvdCollectionDate;
use super::date_posted::datePosted;
use super::identifier::identifier;
use super::image::image;
use super::main_entity_of_page::mainEntityOfPage;
use super::subject_of::subjectOf;

/// * MOD OF: https://pending.schema.org * COMMENT: A CDCPMDRecord is a data structure representing a record in a CDC tabular data format       used for hospital data reporting. See <a href="/docs/cdc-covid.html">documentation</a> for details, and the linked CDC materials for authoritative       definitions used as the source here. * EXTEND FROM: https://schema.org/StructuredValue
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct CDCPMDRecord {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<CDCPMDRecordOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct CDCPMDRecordOptions {
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

    /// collectiondate - Date for which patient counts are reported.
    pub cvd_collection_date: Option<cvdCollectionDate>,

    /// Name of the County of the NHSN facility that this data record applies to. Use <a class="localLink" href="/cvdFacilityId">cvdFacilityId</a> to identify the facility. To provide other details, <a class="localLink" href="/healthcareReportingData">healthcareReportingData</a> can be used on a <a class="localLink" href="/Hospital">Hospital</a> entry.
    pub cvd_facility_county: Option<Text>,

    /// Identifier of the NHSN facility that this data record applies to. Use <a class="localLink" href="/cvdFacilityCounty">cvdFacilityCounty</a> to indicate the county. To provide other details, <a class="localLink" href="/healthcareReportingData">healthcareReportingData</a> can be used on a <a class="localLink" href="/Hospital">Hospital</a> entry.
    pub cvd_facility_id: Option<Text>,

    /// numbeds - HOSPITAL INPATIENT BEDS: Inpatient beds, including all staffed, licensed, and overflow (surge) beds used for inpatients.
    pub cvd_num_beds: Option<Number>,

    /// numbedsocc - HOSPITAL INPATIENT BED OCCUPANCY: Total number of staffed inpatient beds that are occupied.
    pub cvd_num_beds_occ: Option<Number>,

    /// numc19died - DEATHS: Patients with suspected or confirmed COVID-19 who died in the hospital, ED, or any overflow location.
    pub cvd_num_c19_died: Option<Number>,

    /// numc19hopats - HOSPITAL ONSET: Patients hospitalized in an NHSN inpatient care location with onset of suspected or confirmed COVID-19 14 or more days after hospitalization.
    pub cvd_num_c19ho_pats: Option<Number>,

    /// numc19hosppats - HOSPITALIZED: Patients currently hospitalized in an inpatient care location who have suspected or confirmed COVID-19.
    pub cvd_num_c19_hosp_pats: Option<Number>,

    /// numc19mechventpats - HOSPITALIZED and VENTILATED: Patients hospitalized in an NHSN inpatient care location who have suspected or confirmed COVID-19 and are on a mechanical ventilator.
    pub cvd_num_c19_mech_vent_pats: Option<Number>,

    /// numc19ofmechventpats - ED/OVERFLOW and VENTILATED: Patients with suspected or confirmed COVID-19 who are in the ED or any overflow location awaiting an inpatient bed and on a mechanical ventilator.
    pub cvd_num_c19of_mech_vent_pats: Option<Number>,

    /// numc19overflowpats - ED/OVERFLOW: Patients with suspected or confirmed COVID-19 who are in the ED or any overflow location awaiting an inpatient bed.
    pub cvd_num_c19_overflow_pats: Option<Number>,

    /// numicubeds - ICU BEDS: Total number of staffed inpatient intensive care unit (ICU) beds.
    pub cvd_num_icu_beds: Option<Number>,

    /// numicubedsocc - ICU BED OCCUPANCY: Total number of staffed inpatient ICU beds that are occupied.
    pub cvd_num_icu_beds_occ: Option<Number>,

    /// numtotbeds - ALL HOSPITAL BEDS: Total number of all inpatient and outpatient beds, including all staffed, ICU, licensed, and overflow (surge) beds used for inpatients or outpatients.
    pub cvd_num_tot_beds: Option<Number>,

    /// numvent - MECHANICAL VENTILATORS: Total number of ventilators available.
    pub cvd_num_vent: Option<Number>,

    /// numventuse - MECHANICAL VENTILATORS IN USE: Total number of ventilators in use.
    pub cvd_num_vent_use: Option<Number>,

    /// Publication date of an online listing.
    pub date_posted: Option<datePosted>,
}

impl CDCPMDRecord {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
