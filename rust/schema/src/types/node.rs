use crate::prelude::*;

use super::am_radio_channel::AMRadioChannel;
use super::api_reference::APIReference;
use super::abdomen::Abdomen;
use super::about_page::AboutPage;
use super::accept_action::AcceptAction;
use super::accommodation::Accommodation;
use super::accounting_service::AccountingService;
use super::achieve_action::AchieveAction;
use super::action::Action;
use super::action_access_specification::ActionAccessSpecification;
use super::action_status_type::ActionStatusType;
use super::activate_action::ActivateAction;
use super::activation_fee::ActivationFee;
use super::active_action_status::ActiveActionStatus;
use super::active_not_recruiting::ActiveNotRecruiting;
use super::add_action::AddAction;
use super::administrative_area::AdministrativeArea;
use super::adult_entertainment::AdultEntertainment;
use super::adult_oriented_enumeration::AdultOrientedEnumeration;
use super::advertiser_content_article::AdvertiserContentArticle;
use super::aerobic_activity::AerobicActivity;
use super::aggregate_offer::AggregateOffer;
use super::aggregate_rating::AggregateRating;
use super::agree_action::AgreeAction;
use super::airline::Airline;
use super::airport::Airport;
use super::album_release::AlbumRelease;
use super::alcohol_consideration::AlcoholConsideration;
use super::alignment_object::AlignmentObject;
use super::all_wheel_drive_configuration::AllWheelDriveConfiguration;
use super::allergies_health_aspect::AllergiesHealthAspect;
use super::allocate_action::AllocateAction;
use super::amp_story::AmpStory;
use super::amusement_park::AmusementPark;
use super::anaerobic_activity::AnaerobicActivity;
use super::analysis_news_article::AnalysisNewsArticle;
use super::anatomical_structure::AnatomicalStructure;
use super::anatomical_system::AnatomicalSystem;
use super::android_platform::AndroidPlatform;
use super::anesthesia::Anesthesia;
use super::animal_shelter::AnimalShelter;
use super::answer::Answer;
use super::apartment::Apartment;
use super::apartment_complex::ApartmentComplex;
use super::appearance::Appearance;
use super::append_action::AppendAction;
use super::apply_action::ApplyAction;
use super::approved_indication::ApprovedIndication;
use super::aquarium::Aquarium;
use super::archive_component::ArchiveComponent;
use super::archive_organization::ArchiveOrganization;
use super::array::Array;
use super::arrive_action::ArriveAction;
use super::art_gallery::ArtGallery;
use super::artery::Artery;
use super::article::Article;
use super::ask_action::AskAction;
use super::ask_public_news_article::AskPublicNewsArticle;
use super::assess_action::AssessAction;
use super::assign_action::AssignAction;
use super::atlas::Atlas;
use super::attorney::Attorney;
use super::audience::Audience;
use super::audio_object::AudioObject;
use super::audio_object_snapshot::AudioObjectSnapshot;
use super::audiobook::Audiobook;
use super::audiobook_format::AudiobookFormat;
use super::authoritative_legal_value::AuthoritativeLegalValue;
use super::authorize_action::AuthorizeAction;
use super::auto_body_shop::AutoBodyShop;
use super::auto_dealer::AutoDealer;
use super::auto_parts_store::AutoPartsStore;
use super::auto_rental::AutoRental;
use super::auto_repair::AutoRepair;
use super::auto_wash::AutoWash;
use super::automated_teller::AutomatedTeller;
use super::automotive_business::AutomotiveBusiness;
use super::ayurvedic::Ayurvedic;
use super::back_order::BackOrder;
use super::background_news_article::BackgroundNewsArticle;
use super::bacteria::Bacteria;
use super::bakery::Bakery;
use super::balance::Balance;
use super::bank_account::BankAccount;
use super::bank_or_credit_union::BankOrCreditUnion;
use super::bar_or_pub::BarOrPub;
use super::barcode::Barcode;
use super::basic_income::BasicIncome;
use super::beach::Beach;
use super::beauty_salon::BeautySalon;
use super::bed_and_breakfast::BedAndBreakfast;
use super::bed_details::BedDetails;
use super::bed_type::BedType;
use super::befriend_action::BefriendAction;
use super::benefits_health_aspect::BenefitsHealthAspect;
use super::bike_store::BikeStore;
use super::bio_chem_entity::BioChemEntity;
use super::blog::Blog;
use super::blog_posting::BlogPosting;
use super::blood_test::BloodTest;
use super::boarding_policy_type::BoardingPolicyType;
use super::boat_reservation::BoatReservation;
use super::boat_terminal::BoatTerminal;
use super::boat_trip::BoatTrip;
use super::body_measurement_arm::BodyMeasurementArm;
use super::body_measurement_bust::BodyMeasurementBust;
use super::body_measurement_chest::BodyMeasurementChest;
use super::body_measurement_foot::BodyMeasurementFoot;
use super::body_measurement_hand::BodyMeasurementHand;
use super::body_measurement_head::BodyMeasurementHead;
use super::body_measurement_height::BodyMeasurementHeight;
use super::body_measurement_hips::BodyMeasurementHips;
use super::body_measurement_inside_leg::BodyMeasurementInsideLeg;
use super::body_measurement_neck::BodyMeasurementNeck;
use super::body_measurement_type_enumeration::BodyMeasurementTypeEnumeration;
use super::body_measurement_underbust::BodyMeasurementUnderbust;
use super::body_measurement_waist::BodyMeasurementWaist;
use super::body_measurement_weight::BodyMeasurementWeight;
use super::body_of_water::BodyOfWater;
use super::bone::Bone;
use super::book::Book;
use super::book_format_type::BookFormatType;
use super::book_series::BookSeries;
use super::book_store::BookStore;
use super::bookmark_action::BookmarkAction;
use super::boolean::Boolean;
use super::boolean::Boolean;
use super::borrow_action::BorrowAction;
use super::bowling_alley::BowlingAlley;
use super::brain_structure::BrainStructure;
use super::brand::Brand;
use super::breadcrumb_list::BreadcrumbList;
use super::brewery::Brewery;
use super::bridge::Bridge;
use super::broadcast_channel::BroadcastChannel;
use super::broadcast_event::BroadcastEvent;
use super::broadcast_frequency_specification::BroadcastFrequencySpecification;
use super::broadcast_release::BroadcastRelease;
use super::broadcast_service::BroadcastService;
use super::brokerage_account::BrokerageAccount;
use super::buddhist_temple::BuddhistTemple;
use super::bus_or_coach::BusOrCoach;
use super::bus_reservation::BusReservation;
use super::bus_station::BusStation;
use super::bus_stop::BusStop;
use super::bus_trip::BusTrip;
use super::business_audience::BusinessAudience;
use super::business_entity_type::BusinessEntityType;
use super::business_event::BusinessEvent;
use super::business_function::BusinessFunction;
use super::business_support::BusinessSupport;
use super::buy_action::BuyAction;
use super::cdcpmd_record::CDCPMDRecord;
use super::cd_format::CDFormat;
use super::ct::CT;
use super::cable_or_satellite_service::CableOrSatelliteService;
use super::cafe_or_coffee_shop::CafeOrCoffeeShop;
use super::campground::Campground;
use super::camping_pitch::CampingPitch;
use super::canal::Canal;
use super::cancel_action::CancelAction;
use super::car::Car;
use super::car_usage_type::CarUsageType;
use super::cardiovascular::Cardiovascular;
use super::cardiovascular_exam::CardiovascularExam;
use super::case_series::CaseSeries;
use super::casino::Casino;
use super::cassette_format::CassetteFormat;
use super::category_code::CategoryCode;
use super::category_code_set::CategoryCodeSet;
use super::catholic_church::CatholicChurch;
use super::causes_health_aspect::CausesHealthAspect;
use super::cemetery::Cemetery;
use super::chapter::Chapter;
use super::charitable_incorporated_organization::CharitableIncorporatedOrganization;
use super::check_action::CheckAction;
use super::check_in_action::CheckInAction;
use super::check_out_action::CheckOutAction;
use super::checkout_page::CheckoutPage;
use super::chemical_substance::ChemicalSubstance;
use super::child_care::ChildCare;
use super::childrens_event::ChildrensEvent;
use super::chiropractic::Chiropractic;
use super::choose_action::ChooseAction;
use super::church::Church;
use super::city::City;
use super::city_hall::CityHall;
use super::civic_structure::CivicStructure;
use super::claim::Claim;
use super::claim_review::ClaimReview;
use super::class::Class;
use super::cleaning_fee::CleaningFee;
use super::clinician::Clinician;
use super::clip::Clip;
use super::clothing_store::ClothingStore;
use super::co_op::CoOp;
use super::cohort_study::CohortStudy;
use super::collection::Collection;
use super::collection_page::CollectionPage;
use super::college_or_university::CollegeOrUniversity;
use super::comedy_club::ComedyClub;
use super::comedy_event::ComedyEvent;
use super::comic_cover_art::ComicCoverArt;
use super::comic_issue::ComicIssue;
use super::comic_series::ComicSeries;
use super::comic_story::ComicStory;
use super::comment::Comment;
use super::comment_action::CommentAction;
use super::comment_permission::CommentPermission;
use super::communicate_action::CommunicateAction;
use super::community_health::CommunityHealth;
use super::compilation_album::CompilationAlbum;
use super::complete_data_feed::CompleteDataFeed;
use super::completed::Completed;
use super::completed_action_status::CompletedActionStatus;
use super::compound_price_specification::CompoundPriceSpecification;
use super::computer_language::ComputerLanguage;
use super::computer_store::ComputerStore;
use super::confirm_action::ConfirmAction;
use super::consortium::Consortium;
use super::consume_action::ConsumeAction;
use super::contact_page::ContactPage;
use super::contact_point::ContactPoint;
use super::contact_point_option::ContactPointOption;
use super::contagiousness_health_aspect::ContagiousnessHealthAspect;
use super::continent::Continent;
use super::control_action::ControlAction;
use super::convenience_store::ConvenienceStore;
use super::conversation::Conversation;
use super::cook_action::CookAction;
use super::corporation::Corporation;
use super::correction_comment::CorrectionComment;
use super::country::Country;
use super::course::Course;
use super::course_instance::CourseInstance;
use super::courthouse::Courthouse;
use super::cover_art::CoverArt;
use super::covid_testing_facility::CovidTestingFacility;
use super::create_action::CreateAction;
use super::creative_work::CreativeWork;
use super::creative_work_season::CreativeWorkSeason;
use super::creative_work_series::CreativeWorkSeries;
use super::credit_card::CreditCard;
use super::crematorium::Crematorium;
use super::critic_review::CriticReview;
use super::cross_sectional::CrossSectional;
use super::css_selector_type::CssSelectorType;
use super::currency_conversion_service::CurrencyConversionService;
use super::d_dx_element::DDxElement;
use super::dj_mix_album::DJMixAlbum;
use super::dvd_format::DVDFormat;
use super::damaged_condition::DamagedCondition;
use super::dance_event::DanceEvent;
use super::dance_group::DanceGroup;
use super::dangerous_good_consideration::DangerousGoodConsideration;
use super::data_catalog::DataCatalog;
use super::data_download::DataDownload;
use super::data_feed::DataFeed;
use super::data_feed_item::DataFeedItem;
use super::data_type::DataType;
use super::dataset::Dataset;
use super::date::Date;
use super::date_time::DateTime;
use super::day_of_week::DayOfWeek;
use super::day_spa::DaySpa;
use super::deactivate_action::DeactivateAction;
use super::decontextualized_content::DecontextualizedContent;
use super::defence_establishment::DefenceEstablishment;
use super::defined_region::DefinedRegion;
use super::defined_term::DefinedTerm;
use super::defined_term_set::DefinedTermSet;
use super::definitive_legal_value::DefinitiveLegalValue;
use super::delete_action::DeleteAction;
use super::delivery_charge_specification::DeliveryChargeSpecification;
use super::delivery_event::DeliveryEvent;
use super::delivery_method::DeliveryMethod;
use super::delivery_time_settings::DeliveryTimeSettings;
use super::demand::Demand;
use super::demo_album::DemoAlbum;
use super::demo_game_availability::DemoGameAvailability;
use super::dentist::Dentist;
use super::dentistry::Dentistry;
use super::depart_action::DepartAction;
use super::department_store::DepartmentStore;
use super::deposit_account::DepositAccount;
use super::dermatology::Dermatology;
use super::desktop_web_platform::DesktopWebPlatform;
use super::diabetic_diet::DiabeticDiet;
use super::diagnostic::Diagnostic;
use super::diagnostic_lab::DiagnosticLab;
use super::diagnostic_procedure::DiagnosticProcedure;
use super::diet::Diet;
use super::diet_nutrition::DietNutrition;
use super::dietary_supplement::DietarySupplement;
use super::digital_audio_tape_format::DigitalAudioTapeFormat;
use super::digital_document::DigitalDocument;
use super::digital_document_permission::DigitalDocumentPermission;
use super::digital_document_permission_type::DigitalDocumentPermissionType;
use super::digital_format::DigitalFormat;
use super::digital_platform_enumeration::DigitalPlatformEnumeration;
use super::disability_support::DisabilitySupport;
use super::disagree_action::DisagreeAction;
use super::discontinued::Discontinued;
use super::discover_action::DiscoverAction;
use super::discussion_forum_posting::DiscussionForumPosting;
use super::dislike_action::DislikeAction;
use super::distance::Distance;
use super::distance_fee::DistanceFee;
use super::distillery::Distillery;
use super::donate_action::DonateAction;
use super::dose_schedule::DoseSchedule;
use super::double_blinded_trial::DoubleBlindedTrial;
use super::download_action::DownloadAction;
use super::downpayment::Downpayment;
use super::draw_action::DrawAction;
use super::drawing::Drawing;
use super::drink_action::DrinkAction;
use super::drive_wheel_configuration_value::DriveWheelConfigurationValue;
use super::driving_school_vehicle_usage::DrivingSchoolVehicleUsage;
use super::drug::Drug;
use super::drug_class::DrugClass;
use super::drug_cost::DrugCost;
use super::drug_cost_category::DrugCostCategory;
use super::drug_legal_status::DrugLegalStatus;
use super::drug_pregnancy_category::DrugPregnancyCategory;
use super::drug_prescription_status::DrugPrescriptionStatus;
use super::drug_strength::DrugStrength;
use super::dry_cleaning_or_laundry::DryCleaningOrLaundry;
use super::duration::Duration;
use super::e_book::EBook;
use super::ep_release::EPRelease;
use super::eu_energy_efficiency_category_a::EUEnergyEfficiencyCategoryA;
use super::eu_energy_efficiency_category_a1_plus::EUEnergyEfficiencyCategoryA1Plus;
use super::eu_energy_efficiency_category_a2_plus::EUEnergyEfficiencyCategoryA2Plus;
use super::eu_energy_efficiency_category_a3_plus::EUEnergyEfficiencyCategoryA3Plus;
use super::eu_energy_efficiency_category_b::EUEnergyEfficiencyCategoryB;
use super::eu_energy_efficiency_category_c::EUEnergyEfficiencyCategoryC;
use super::eu_energy_efficiency_category_d::EUEnergyEfficiencyCategoryD;
use super::eu_energy_efficiency_category_e::EUEnergyEfficiencyCategoryE;
use super::eu_energy_efficiency_category_f::EUEnergyEfficiencyCategoryF;
use super::eu_energy_efficiency_category_g::EUEnergyEfficiencyCategoryG;
use super::eu_energy_efficiency_enumeration::EUEnergyEfficiencyEnumeration;
use super::ear::Ear;
use super::eat_action::EatAction;
use super::edited_or_cropped_content::EditedOrCroppedContent;
use super::education_event::EducationEvent;
use super::educational_audience::EducationalAudience;
use super::educational_occupational_credential::EducationalOccupationalCredential;
use super::educational_occupational_program::EducationalOccupationalProgram;
use super::educational_organization::EducationalOrganization;
use super::effectiveness_health_aspect::EffectivenessHealthAspect;
use super::electrician::Electrician;
use super::electronics_store::ElectronicsStore;
use super::elementary_school::ElementarySchool;
use super::email_message::EmailMessage;
use super::embassy::Embassy;
use super::emergency::Emergency;
use super::emergency_service::EmergencyService;
use super::employee_role::EmployeeRole;
use super::employer_aggregate_rating::EmployerAggregateRating;
use super::employer_review::EmployerReview;
use super::employment_agency::EmploymentAgency;
use super::endocrine::Endocrine;
use super::endorse_action::EndorseAction;
use super::endorsement_rating::EndorsementRating;
use super::energy::Energy;
use super::energy_consumption_details::EnergyConsumptionDetails;
use super::energy_efficiency_enumeration::EnergyEfficiencyEnumeration;
use super::energy_star_certified::EnergyStarCertified;
use super::energy_star_energy_efficiency_enumeration::EnergyStarEnergyEfficiencyEnumeration;
use super::engine_specification::EngineSpecification;
use super::enrolling_by_invitation::EnrollingByInvitation;
use super::entertainment_business::EntertainmentBusiness;
use super::entry_point::EntryPoint;
use super::enumeration::Enumeration;
use super::episode::Episode;
use super::event::Event;
use super::event_attendance_mode_enumeration::EventAttendanceModeEnumeration;
use super::event_cancelled::EventCancelled;
use super::event_moved_online::EventMovedOnline;
use super::event_postponed::EventPostponed;
use super::event_rescheduled::EventRescheduled;
use super::event_reservation::EventReservation;
use super::event_scheduled::EventScheduled;
use super::event_series::EventSeries;
use super::event_status_type::EventStatusType;
use super::event_venue::EventVenue;
use super::evidence_level_a::EvidenceLevelA;
use super::evidence_level_b::EvidenceLevelB;
use super::evidence_level_c::EvidenceLevelC;
use super::exchange_rate_specification::ExchangeRateSpecification;
use super::exchange_refund::ExchangeRefund;
use super::exercise_action::ExerciseAction;
use super::exercise_gym::ExerciseGym;
use super::exercise_plan::ExercisePlan;
use super::exhibition_event::ExhibitionEvent;
use super::eye::Eye;
use super::faq_page::FAQPage;
use super::fd_acategory_a::FDAcategoryA;
use super::fd_acategory_b::FDAcategoryB;
use super::fd_acategory_c::FDAcategoryC;
use super::fd_acategory_d::FDAcategoryD;
use super::fd_acategory_x::FDAcategoryX;
use super::fd_anot_evaluated::FDAnotEvaluated;
use super::fm_radio_channel::FMRadioChannel;
use super::failed_action_status::FailedActionStatus;
use super::false::False;
use super::fast_food_restaurant::FastFoodRestaurant;
use super::female::Female;
use super::festival::Festival;
use super::film_action::FilmAction;
use super::financial_product::FinancialProduct;
use super::financial_service::FinancialService;
use super::find_action::FindAction;
use super::fire_station::FireStation;
use super::flexibility::Flexibility;
use super::flight::Flight;
use super::flight_reservation::FlightReservation;
use super::float::Float;
use super::floor_plan::FloorPlan;
use super::florist::Florist;
use super::follow_action::FollowAction;
use super::food_establishment::FoodEstablishment;
use super::food_establishment_reservation::FoodEstablishmentReservation;
use super::food_event::FoodEvent;
use super::food_service::FoodService;
use super::four_wheel_drive_configuration::FourWheelDriveConfiguration;
use super::free_return::FreeReturn;
use super::friday::Friday;
use super::front_wheel_drive_configuration::FrontWheelDriveConfiguration;
use super::full_game_availability::FullGameAvailability;
use super::full_refund::FullRefund;
use super::funding_agency::FundingAgency;
use super::funding_scheme::FundingScheme;
use super::fungus::Fungus;
use super::furniture_store::FurnitureStore;
use super::game::Game;
use super::game_availability_enumeration::GameAvailabilityEnumeration;
use super::game_play_mode::GamePlayMode;
use super::game_server::GameServer;
use super::game_server_status::GameServerStatus;
use super::garden_store::GardenStore;
use super::gas_station::GasStation;
use super::gastroenterologic::Gastroenterologic;
use super::gated_residence_community::GatedResidenceCommunity;
use super::gender_type::GenderType;
use super::gene::Gene;
use super::general_contractor::GeneralContractor;
use super::generic_web_platform::GenericWebPlatform;
use super::genetic::Genetic;
use super::genitourinary::Genitourinary;
use super::geo_circle::GeoCircle;
use super::geo_coordinates::GeoCoordinates;
use super::geo_shape::GeoShape;
use super::geospatial_geometry::GeospatialGeometry;
use super::geriatric::Geriatric;
use super::getting_access_health_aspect::GettingAccessHealthAspect;
use super::give_action::GiveAction;
use super::gluten_free_diet::GlutenFreeDiet;
use super::golf_course::GolfCourse;
use super::government_benefits_type::GovernmentBenefitsType;
use super::government_building::GovernmentBuilding;
use super::government_office::GovernmentOffice;
use super::government_organization::GovernmentOrganization;
use super::government_permit::GovernmentPermit;
use super::government_service::GovernmentService;
use super::grant::Grant;
use super::graphic_novel::GraphicNovel;
use super::grocery_store::GroceryStore;
use super::group_boarding_policy::GroupBoardingPolicy;
use super::guide::Guide;
use super::gynecologic::Gynecologic;
use super::hvac_business::HVACBusiness;
use super::hackathon::Hackathon;
use super::hair_salon::HairSalon;
use super::halal_diet::HalalDiet;
use super::hardcover::Hardcover;
use super::hardware_store::HardwareStore;
use super::head::Head;
use super::health_and_beauty_business::HealthAndBeautyBusiness;
use super::health_aspect_enumeration::HealthAspectEnumeration;
use super::health_care::HealthCare;
use super::health_club::HealthClub;
use super::health_insurance_plan::HealthInsurancePlan;
use super::health_plan_cost_sharing_specification::HealthPlanCostSharingSpecification;
use super::health_plan_formulary::HealthPlanFormulary;
use super::health_plan_network::HealthPlanNetwork;
use super::health_topic_content::HealthTopicContent;
use super::healthcare_consideration::HealthcareConsideration;
use super::hearing_impaired_supported::HearingImpairedSupported;
use super::hematologic::Hematologic;
use super::high_school::HighSchool;
use super::hindu_diet::HinduDiet;
use super::hindu_temple::HinduTemple;
use super::hobby_shop::HobbyShop;
use super::home_and_construction_business::HomeAndConstructionBusiness;
use super::home_goods_store::HomeGoodsStore;
use super::homeopathic::Homeopathic;
use super::hospital::Hospital;
use super::hostel::Hostel;
use super::hotel::Hotel;
use super::hotel_room::HotelRoom;
use super::house::House;
use super::house_painter::HousePainter;
use super::how_it_works_health_aspect::HowItWorksHealthAspect;
use super::how_or_where_health_aspect::HowOrWhereHealthAspect;
use super::how_to::HowTo;
use super::how_to_direction::HowToDirection;
use super::how_to_item::HowToItem;
use super::how_to_section::HowToSection;
use super::how_to_step::HowToStep;
use super::how_to_supply::HowToSupply;
use super::how_to_tip::HowToTip;
use super::how_to_tool::HowToTool;
use super::hyper_toc::HyperToc;
use super::hyper_toc_entry::HyperTocEntry;
use super::ios_platform::IOSPlatform;
use super::ice_cream_shop::IceCreamShop;
use super::ignore_action::IgnoreAction;
use super::image_gallery::ImageGallery;
use super::image_object::ImageObject;
use super::image_object_snapshot::ImageObjectSnapshot;
use super::imaging_test::ImagingTest;
use super::in_force::InForce;
use super::in_stock::InStock;
use super::in_store_only::InStoreOnly;
use super::individual_product::IndividualProduct;
use super::infectious::Infectious;
use super::infectious_agent_class::InfectiousAgentClass;
use super::infectious_disease::InfectiousDisease;
use super::inform_action::InformAction;
use super::ingredients_health_aspect::IngredientsHealthAspect;
use super::insert_action::InsertAction;
use super::install_action::InstallAction;
use super::installment::Installment;
use super::insurance_agency::InsuranceAgency;
use super::intangible::Intangible;
use super::integer::Integer;
use super::integer::Integer;
use super::interact_action::InteractAction;
use super::interaction_counter::InteractionCounter;
use super::international_trial::InternationalTrial;
use super::internet_cafe::InternetCafe;
use super::investment_fund::InvestmentFund;
use super::investment_or_deposit::InvestmentOrDeposit;
use super::invite_action::InviteAction;
use super::invoice::Invoice;
use super::invoice_price::InvoicePrice;
use super::item_availability::ItemAvailability;
use super::item_list::ItemList;
use super::item_list_order_ascending::ItemListOrderAscending;
use super::item_list_order_descending::ItemListOrderDescending;
use super::item_list_order_type::ItemListOrderType;
use super::item_list_unordered::ItemListUnordered;
use super::item_page::ItemPage;
use super::jewelry_store::JewelryStore;
use super::job_posting::JobPosting;
use super::join_action::JoinAction;
use super::joint::Joint;
use super::kosher_diet::KosherDiet;
use super::laboratory_science::LaboratoryScience;
use super::lake_body_of_water::LakeBodyOfWater;
use super::landform::Landform;
use super::landmarks_or_historical_buildings::LandmarksOrHistoricalBuildings;
use super::language::Language;
use super::laser_disc_format::LaserDiscFormat;
use super::learning_resource::LearningResource;
use super::leave_action::LeaveAction;
use super::left_hand_driving::LeftHandDriving;
use super::legal_force_status::LegalForceStatus;
use super::legal_service::LegalService;
use super::legal_value_level::LegalValueLevel;
use super::legislation::Legislation;
use super::legislation_object::LegislationObject;
use super::legislative_building::LegislativeBuilding;
use super::leisure_time_activity::LeisureTimeActivity;
use super::lend_action::LendAction;
use super::library::Library;
use super::library_system::LibrarySystem;
use super::lifestyle_modification::LifestyleModification;
use super::ligament::Ligament;
use super::like_action::LikeAction;
use super::limited_availability::LimitedAvailability;
use super::limited_by_guarantee_charity::LimitedByGuaranteeCharity;
use super::link_role::LinkRole;
use super::liquor_store::LiquorStore;
use super::list_item::ListItem;
use super::list_price::ListPrice;
use super::listen_action::ListenAction;
use super::literary_event::LiteraryEvent;
use super::live_album::LiveAlbum;
use super::live_blog_posting::LiveBlogPosting;
use super::living_with_health_aspect::LivingWithHealthAspect;
use super::loan_or_credit::LoanOrCredit;
use super::local_business::LocalBusiness;
use super::location_feature_specification::LocationFeatureSpecification;
use super::locker_delivery::LockerDelivery;
use super::locksmith::Locksmith;
use super::lodging_business::LodgingBusiness;
use super::lodging_reservation::LodgingReservation;
use super::longitudinal::Longitudinal;
use super::lose_action::LoseAction;
use super::low_calorie_diet::LowCalorieDiet;
use super::low_fat_diet::LowFatDiet;
use super::low_lactose_diet::LowLactoseDiet;
use super::low_salt_diet::LowSaltDiet;
use super::lung::Lung;
use super::lymphatic_vessel::LymphaticVessel;
use super::mri::MRI;
use super::msrp::MSRP;
use super::male::Male;
use super::manuscript::Manuscript;
use super::map::Map;
use super::map_category_type::MapCategoryType;
use super::marry_action::MarryAction;
use super::mass::Mass;
use super::math_solver::MathSolver;
use super::maximum_dose_schedule::MaximumDoseSchedule;
use super::may_treat_health_aspect::MayTreatHealthAspect;
use super::measurement_type_enumeration::MeasurementTypeEnumeration;
use super::media_gallery::MediaGallery;
use super::media_manipulation_rating_enumeration::MediaManipulationRatingEnumeration;
use super::media_object::MediaObject;
use super::media_review::MediaReview;
use super::media_review_item::MediaReviewItem;
use super::media_subscription::MediaSubscription;
use super::medical_audience::MedicalAudience;
use super::medical_audience_type::MedicalAudienceType;
use super::medical_business::MedicalBusiness;
use super::medical_cause::MedicalCause;
use super::medical_clinic::MedicalClinic;
use super::medical_code::MedicalCode;
use super::medical_condition::MedicalCondition;
use super::medical_condition_stage::MedicalConditionStage;
use super::medical_contraindication::MedicalContraindication;
use super::medical_device::MedicalDevice;
use super::medical_device_purpose::MedicalDevicePurpose;
use super::medical_entity::MedicalEntity;
use super::medical_enumeration::MedicalEnumeration;
use super::medical_evidence_level::MedicalEvidenceLevel;
use super::medical_guideline::MedicalGuideline;
use super::medical_guideline_contraindication::MedicalGuidelineContraindication;
use super::medical_guideline_recommendation::MedicalGuidelineRecommendation;
use super::medical_imaging_technique::MedicalImagingTechnique;
use super::medical_indication::MedicalIndication;
use super::medical_intangible::MedicalIntangible;
use super::medical_observational_study::MedicalObservationalStudy;
use super::medical_observational_study_design::MedicalObservationalStudyDesign;
use super::medical_organization::MedicalOrganization;
use super::medical_procedure::MedicalProcedure;
use super::medical_procedure_type::MedicalProcedureType;
use super::medical_researcher::MedicalResearcher;
use super::medical_risk_calculator::MedicalRiskCalculator;
use super::medical_risk_estimator::MedicalRiskEstimator;
use super::medical_risk_factor::MedicalRiskFactor;
use super::medical_risk_score::MedicalRiskScore;
use super::medical_scholarly_article::MedicalScholarlyArticle;
use super::medical_sign::MedicalSign;
use super::medical_sign_or_symptom::MedicalSignOrSymptom;
use super::medical_specialty::MedicalSpecialty;
use super::medical_study::MedicalStudy;
use super::medical_study_status::MedicalStudyStatus;
use super::medical_symptom::MedicalSymptom;
use super::medical_test::MedicalTest;
use super::medical_test_panel::MedicalTestPanel;
use super::medical_therapy::MedicalTherapy;
use super::medical_trial::MedicalTrial;
use super::medical_trial_design::MedicalTrialDesign;
use super::medical_web_page::MedicalWebPage;
use super::medicine_system::MedicineSystem;
use super::meeting_room::MeetingRoom;
use super::mens_clothing_store::MensClothingStore;
use super::menu::Menu;
use super::menu_item::MenuItem;
use super::menu_section::MenuSection;
use super::merchant_return_enumeration::MerchantReturnEnumeration;
use super::merchant_return_finite_return_window::MerchantReturnFiniteReturnWindow;
use super::merchant_return_not_permitted::MerchantReturnNotPermitted;
use super::merchant_return_policy::MerchantReturnPolicy;
use super::merchant_return_policy_seasonal_override::MerchantReturnPolicySeasonalOverride;
use super::merchant_return_unlimited_window::MerchantReturnUnlimitedWindow;
use super::merchant_return_unspecified::MerchantReturnUnspecified;
use super::message::Message;
use super::middle_school::MiddleSchool;
use super::midwifery::Midwifery;
use super::minimum_advertised_price::MinimumAdvertisedPrice;
use super::misconceptions_health_aspect::MisconceptionsHealthAspect;
use super::mixed_event_attendance_mode::MixedEventAttendanceMode;
use super::mixtape_album::MixtapeAlbum;
use super::mobile_application::MobileApplication;
use super::mobile_phone_store::MobilePhoneStore;
use super::mobile_web_platform::MobileWebPlatform;
use super::model_3d::Model3D;
use super::molecular_entity::MolecularEntity;
use super::monday::Monday;
use super::monetary_amount::MonetaryAmount;
use super::monetary_amount_distribution::MonetaryAmountDistribution;
use super::monetary_grant::MonetaryGrant;
use super::money_transfer::MoneyTransfer;
use super::mortgage_loan::MortgageLoan;
use super::mosque::Mosque;
use super::motel::Motel;
use super::motorcycle::Motorcycle;
use super::motorcycle_dealer::MotorcycleDealer;
use super::motorcycle_repair::MotorcycleRepair;
use super::motorized_bicycle::MotorizedBicycle;
use super::mountain::Mountain;
use super::move_action::MoveAction;
use super::movie::Movie;
use super::movie_clip::MovieClip;
use super::movie_rental_store::MovieRentalStore;
use super::movie_series::MovieSeries;
use super::movie_theater::MovieTheater;
use super::moving_company::MovingCompany;
use super::multi_center_trial::MultiCenterTrial;
use super::multi_player::MultiPlayer;
use super::multicellular_parasite::MulticellularParasite;
use super::muscle::Muscle;
use super::musculoskeletal::Musculoskeletal;
use super::musculoskeletal_exam::MusculoskeletalExam;
use super::museum::Museum;
use super::music_album::MusicAlbum;
use super::music_album_production_type::MusicAlbumProductionType;
use super::music_album_release_type::MusicAlbumReleaseType;
use super::music_composition::MusicComposition;
use super::music_event::MusicEvent;
use super::music_group::MusicGroup;
use super::music_playlist::MusicPlaylist;
use super::music_recording::MusicRecording;
use super::music_release::MusicRelease;
use super::music_release_format_type::MusicReleaseFormatType;
use super::music_store::MusicStore;
use super::music_venue::MusicVenue;
use super::music_video_object::MusicVideoObject;
use super::ngo::NGO;
use super::nl_nonprofit_type::NLNonprofitType;
use super::nail_salon::NailSalon;
use super::narcotic_consideration::NarcoticConsideration;
use super::neck::Neck;
use super::nerve::Nerve;
use super::neuro::Neuro;
use super::neurologic::Neurologic;
use super::new_condition::NewCondition;
use super::news_article::NewsArticle;
use super::news_media_organization::NewsMediaOrganization;
use super::newspaper::Newspaper;
use super::night_club::NightClub;
use super::noninvasive_procedure::NoninvasiveProcedure;
use super::nonprofit_50_1a::Nonprofit501a;
use super::nonprofit_50_1c_1::Nonprofit501c1;
use super::nonprofit_50_1c_10::Nonprofit501c10;
use super::nonprofit_50_1c_11::Nonprofit501c11;
use super::nonprofit_50_1c_12::Nonprofit501c12;
use super::nonprofit_50_1c_13::Nonprofit501c13;
use super::nonprofit_50_1c_14::Nonprofit501c14;
use super::nonprofit_50_1c_15::Nonprofit501c15;
use super::nonprofit_50_1c_16::Nonprofit501c16;
use super::nonprofit_50_1c_17::Nonprofit501c17;
use super::nonprofit_50_1c_18::Nonprofit501c18;
use super::nonprofit_50_1c_19::Nonprofit501c19;
use super::nonprofit_50_1c_2::Nonprofit501c2;
use super::nonprofit_50_1c_20::Nonprofit501c20;
use super::nonprofit_50_1c_21::Nonprofit501c21;
use super::nonprofit_50_1c_22::Nonprofit501c22;
use super::nonprofit_50_1c_23::Nonprofit501c23;
use super::nonprofit_50_1c_24::Nonprofit501c24;
use super::nonprofit_50_1c_25::Nonprofit501c25;
use super::nonprofit_50_1c_26::Nonprofit501c26;
use super::nonprofit_50_1c_27::Nonprofit501c27;
use super::nonprofit_50_1c_28::Nonprofit501c28;
use super::nonprofit_50_1c_3::Nonprofit501c3;
use super::nonprofit_50_1c_4::Nonprofit501c4;
use super::nonprofit_50_1c_5::Nonprofit501c5;
use super::nonprofit_50_1c_6::Nonprofit501c6;
use super::nonprofit_50_1c_7::Nonprofit501c7;
use super::nonprofit_50_1c_8::Nonprofit501c8;
use super::nonprofit_50_1c_9::Nonprofit501c9;
use super::nonprofit_50_1d::Nonprofit501d;
use super::nonprofit_50_1e::Nonprofit501e;
use super::nonprofit_50_1f::Nonprofit501f;
use super::nonprofit_50_1k::Nonprofit501k;
use super::nonprofit_50_1n::Nonprofit501n;
use super::nonprofit_50_1q::Nonprofit501q;
use super::nonprofit_527::Nonprofit527;
use super::nonprofit_anbi::NonprofitANBI;
use super::nonprofit_sbbi::NonprofitSBBI;
use super::nonprofit_type::NonprofitType;
use super::nose::Nose;
use super::not_in_force::NotInForce;
use super::not_yet_recruiting::NotYetRecruiting;
use super::notary::Notary;
use super::note_digital_document::NoteDigitalDocument;
use super::null::Null;
use super::number::Number;
use super::number::Number;
use super::nursing::Nursing;
use super::nutrition_information::NutritionInformation;
use super::otc::OTC;
use super::object::Object;
use super::observation::Observation;
use super::observational::Observational;
use super::obstetric::Obstetric;
use super::occupation::Occupation;
use super::occupational_activity::OccupationalActivity;
use super::occupational_experience_requirements::OccupationalExperienceRequirements;
use super::occupational_therapy::OccupationalTherapy;
use super::ocean_body_of_water::OceanBodyOfWater;
use super::offer::Offer;
use super::offer_catalog::OfferCatalog;
use super::offer_for_lease::OfferForLease;
use super::offer_for_purchase::OfferForPurchase;
use super::offer_item_condition::OfferItemCondition;
use super::offer_shipping_details::OfferShippingDetails;
use super::office_equipment_store::OfficeEquipmentStore;
use super::official_legal_value::OfficialLegalValue;
use super::offline_event_attendance_mode::OfflineEventAttendanceMode;
use super::offline_permanently::OfflinePermanently;
use super::offline_temporarily::OfflineTemporarily;
use super::on_demand_event::OnDemandEvent;
use super::on_site_pickup::OnSitePickup;
use super::oncologic::Oncologic;
use super::one_time_payments::OneTimePayments;
use super::online::Online;
use super::online_business::OnlineBusiness;
use super::online_event_attendance_mode::OnlineEventAttendanceMode;
use super::online_full::OnlineFull;
use super::online_only::OnlineOnly;
use super::online_store::OnlineStore;
use super::open_trial::OpenTrial;
use super::opening_hours_specification::OpeningHoursSpecification;
use super::opinion_news_article::OpinionNewsArticle;
use super::optician::Optician;
use super::optometric::Optometric;
use super::order::Order;
use super::order_action::OrderAction;
use super::order_cancelled::OrderCancelled;
use super::order_delivered::OrderDelivered;
use super::order_in_transit::OrderInTransit;
use super::order_item::OrderItem;
use super::order_payment_due::OrderPaymentDue;
use super::order_pickup_available::OrderPickupAvailable;
use super::order_problem::OrderProblem;
use super::order_processing::OrderProcessing;
use super::order_returned::OrderReturned;
use super::order_status::OrderStatus;
use super::organization::Organization;
use super::organization_role::OrganizationRole;
use super::organize_action::OrganizeAction;
use super::original_media_content::OriginalMediaContent;
use super::original_shipping_fees::OriginalShippingFees;
use super::osteopathic::Osteopathic;
use super::otolaryngologic::Otolaryngologic;
use super::out_of_stock::OutOfStock;
use super::outlet_store::OutletStore;
use super::overview_health_aspect::OverviewHealthAspect;
use super::ownership_info::OwnershipInfo;
use super::pet::PET;
use super::paid_leave::PaidLeave;
use super::paint_action::PaintAction;
use super::painting::Painting;
use super::palliative_procedure::PalliativeProcedure;
use super::paperback::Paperback;
use super::parcel_delivery::ParcelDelivery;
use super::parcel_service::ParcelService;
use super::parent_audience::ParentAudience;
use super::parental_support::ParentalSupport;
use super::park::Park;
use super::parking_facility::ParkingFacility;
use super::parking_map::ParkingMap;
use super::partially_in_force::PartiallyInForce;
use super::pathology::Pathology;
use super::pathology_test::PathologyTest;
use super::patient::Patient;
use super::patient_experience_health_aspect::PatientExperienceHealthAspect;
use super::pawn_shop::PawnShop;
use super::pay_action::PayAction;
use super::payment_automatically_applied::PaymentAutomaticallyApplied;
use super::payment_card::PaymentCard;
use super::payment_charge_specification::PaymentChargeSpecification;
use super::payment_complete::PaymentComplete;
use super::payment_declined::PaymentDeclined;
use super::payment_due::PaymentDue;
use super::payment_method::PaymentMethod;
use super::payment_past_due::PaymentPastDue;
use super::payment_service::PaymentService;
use super::payment_status_type::PaymentStatusType;
use super::pediatric::Pediatric;
use super::people_audience::PeopleAudience;
use super::percutaneous_procedure::PercutaneousProcedure;
use super::perform_action::PerformAction;
use super::performance_role::PerformanceRole;
use super::performing_arts_theater::PerformingArtsTheater;
use super::performing_group::PerformingGroup;
use super::periodical::Periodical;
use super::permit::Permit;
use super::person::Person;
use super::pet_store::PetStore;
use super::pharmacy::Pharmacy;
use super::pharmacy_specialty::PharmacySpecialty;
use super::photograph::Photograph;
use super::photograph_action::PhotographAction;
use super::physical_activity::PhysicalActivity;
use super::physical_activity_category::PhysicalActivityCategory;
use super::physical_exam::PhysicalExam;
use super::physical_therapy::PhysicalTherapy;
use super::physician::Physician;
use super::physiotherapy::Physiotherapy;
use super::place::Place;
use super::place_of_worship::PlaceOfWorship;
use super::placebo_controlled_trial::PlaceboControlledTrial;
use super::plan_action::PlanAction;
use super::plastic_surgery::PlasticSurgery;
use super::play::Play;
use super::play_action::PlayAction;
use super::play_game_action::PlayGameAction;
use super::playground::Playground;
use super::plumber::Plumber;
use super::podcast_episode::PodcastEpisode;
use super::podcast_season::PodcastSeason;
use super::podcast_series::PodcastSeries;
use super::podiatric::Podiatric;
use super::police_station::PoliceStation;
use super::pond::Pond;
use super::post_office::PostOffice;
use super::postal_address::PostalAddress;
use super::postal_code_range_specification::PostalCodeRangeSpecification;
use super::poster::Poster;
use super::potential_action_status::PotentialActionStatus;
use super::pre_order::PreOrder;
use super::pre_order_action::PreOrderAction;
use super::pre_sale::PreSale;
use super::pregnancy_health_aspect::PregnancyHealthAspect;
use super::prepend_action::PrependAction;
use super::preschool::Preschool;
use super::prescription_only::PrescriptionOnly;
use super::presentation_digital_document::PresentationDigitalDocument;
use super::prevention_health_aspect::PreventionHealthAspect;
use super::prevention_indication::PreventionIndication;
use super::price_component_type_enumeration::PriceComponentTypeEnumeration;
use super::price_specification::PriceSpecification;
use super::price_type_enumeration::PriceTypeEnumeration;
use super::primary_care::PrimaryCare;
use super::prion::Prion;
use super::product::Product;
use super::product_collection::ProductCollection;
use super::product_group::ProductGroup;
use super::product_model::ProductModel;
use super::professional_service::ProfessionalService;
use super::profile_page::ProfilePage;
use super::prognosis_health_aspect::PrognosisHealthAspect;
use super::program_membership::ProgramMembership;
use super::project::Project;
use super::pronounceable_text::PronounceableText;
use super::property::Property;
use super::property_value::PropertyValue;
use super::property_value_specification::PropertyValueSpecification;
use super::protein::Protein;
use super::protozoa::Protozoa;
use super::psychiatric::Psychiatric;
use super::psychological_treatment::PsychologicalTreatment;
use super::public_health::PublicHealth;
use super::public_holidays::PublicHolidays;
use super::public_swimming_pool::PublicSwimmingPool;
use super::public_toilet::PublicToilet;
use super::publication_event::PublicationEvent;
use super::publication_issue::PublicationIssue;
use super::publication_volume::PublicationVolume;
use super::pulmonary::Pulmonary;
use super::qa_page::QAPage;
use super::qualitative_value::QualitativeValue;
use super::quantitative_value::QuantitativeValue;
use super::quantitative_value_distribution::QuantitativeValueDistribution;
use super::quantity::Quantity;
use super::question::Question;
use super::quiz::Quiz;
use super::quotation::Quotation;
use super::quote_action::QuoteAction;
use super::rv_park::RVPark;
use super::radiation_therapy::RadiationTherapy;
use super::radio_broadcast_service::RadioBroadcastService;
use super::radio_channel::RadioChannel;
use super::radio_clip::RadioClip;
use super::radio_episode::RadioEpisode;
use super::radio_season::RadioSeason;
use super::radio_series::RadioSeries;
use super::radio_station::RadioStation;
use super::radiography::Radiography;
use super::randomized_trial::RandomizedTrial;
use super::rating::Rating;
use super::react_action::ReactAction;
use super::read_action::ReadAction;
use super::read_permission::ReadPermission;
use super::real_estate_agent::RealEstateAgent;
use super::real_estate_listing::RealEstateListing;
use super::rear_wheel_drive_configuration::RearWheelDriveConfiguration;
use super::receive_action::ReceiveAction;
use super::recipe::Recipe;
use super::recommendation::Recommendation;
use super::recommended_dose_schedule::RecommendedDoseSchedule;
use super::recruiting::Recruiting;
use super::recycling_center::RecyclingCenter;
use super::reduced_relevance_for_children_consideration::ReducedRelevanceForChildrenConsideration;
use super::refund_type_enumeration::RefundTypeEnumeration;
use super::refurbished_condition::RefurbishedCondition;
use super::register_action::RegisterAction;
use super::registry::Registry;
use super::reimbursement_cap::ReimbursementCap;
use super::reject_action::RejectAction;
use super::related_topics_health_aspect::RelatedTopicsHealthAspect;
use super::remix_album::RemixAlbum;
use super::renal::Renal;
use super::rent_action::RentAction;
use super::rental_car_reservation::RentalCarReservation;
use super::rental_vehicle_usage::RentalVehicleUsage;
use super::repayment_specification::RepaymentSpecification;
use super::replace_action::ReplaceAction;
use super::reply_action::ReplyAction;
use super::report::Report;
use super::reportage_news_article::ReportageNewsArticle;
use super::reported_dose_schedule::ReportedDoseSchedule;
use super::research_organization::ResearchOrganization;
use super::research_project::ResearchProject;
use super::researcher::Researcher;
use super::reservation::Reservation;
use super::reservation_cancelled::ReservationCancelled;
use super::reservation_confirmed::ReservationConfirmed;
use super::reservation_hold::ReservationHold;
use super::reservation_package::ReservationPackage;
use super::reservation_pending::ReservationPending;
use super::reservation_status_type::ReservationStatusType;
use super::reserve_action::ReserveAction;
use super::reservoir::Reservoir;
use super::residence::Residence;
use super::resort::Resort;
use super::respiratory_therapy::RespiratoryTherapy;
use super::restaurant::Restaurant;
use super::restocking_fees::RestockingFees;
use super::restricted_diet::RestrictedDiet;
use super::results_available::ResultsAvailable;
use super::results_not_available::ResultsNotAvailable;
use super::resume_action::ResumeAction;
use super::retail::Retail;
use super::return_action::ReturnAction;
use super::return_at_kiosk::ReturnAtKiosk;
use super::return_by_mail::ReturnByMail;
use super::return_fees_customer_responsibility::ReturnFeesCustomerResponsibility;
use super::return_fees_enumeration::ReturnFeesEnumeration;
use super::return_in_store::ReturnInStore;
use super::return_label_customer_responsibility::ReturnLabelCustomerResponsibility;
use super::return_label_download_and_print::ReturnLabelDownloadAndPrint;
use super::return_label_in_box::ReturnLabelInBox;
use super::return_label_source_enumeration::ReturnLabelSourceEnumeration;
use super::return_method_enumeration::ReturnMethodEnumeration;
use super::return_shipping_fees::ReturnShippingFees;
use super::review::Review;
use super::review_action::ReviewAction;
use super::review_news_article::ReviewNewsArticle;
use super::rheumatologic::Rheumatologic;
use super::right_hand_driving::RightHandDriving;
use super::risks_or_complications_health_aspect::RisksOrComplicationsHealthAspect;
use super::river_body_of_water::RiverBodyOfWater;
use super::role::Role;
use super::roofing_contractor::RoofingContractor;
use super::room::Room;
use super::rsvp_action::RsvpAction;
use super::rsvp_response_maybe::RsvpResponseMaybe;
use super::rsvp_response_no::RsvpResponseNo;
use super::rsvp_response_type::RsvpResponseType;
use super::rsvp_response_yes::RsvpResponseYes;
use super::srp::SRP;
use super::safety_health_aspect::SafetyHealthAspect;
use super::sale_event::SaleEvent;
use super::sale_price::SalePrice;
use super::satire_or_parody_content::SatireOrParodyContent;
use super::satirical_article::SatiricalArticle;
use super::saturday::Saturday;
use super::schedule::Schedule;
use super::schedule_action::ScheduleAction;
use super::scholarly_article::ScholarlyArticle;
use super::school::School;
use super::school_district::SchoolDistrict;
use super::screening_event::ScreeningEvent;
use super::screening_health_aspect::ScreeningHealthAspect;
use super::sculpture::Sculpture;
use super::sea_body_of_water::SeaBodyOfWater;
use super::search_action::SearchAction;
use super::search_rescue_organization::SearchRescueOrganization;
use super::search_results_page::SearchResultsPage;
use super::seat::Seat;
use super::seating_map::SeatingMap;
use super::see_doctor_health_aspect::SeeDoctorHealthAspect;
use super::seek_to_action::SeekToAction;
use super::self_care_health_aspect::SelfCareHealthAspect;
use super::self_storage::SelfStorage;
use super::sell_action::SellAction;
use super::send_action::SendAction;
use super::series::Series;
use super::service::Service;
use super::service_channel::ServiceChannel;
use super::sexual_content_consideration::SexualContentConsideration;
use super::share_action::ShareAction;
use super::sheet_music::SheetMusic;
use super::shipping_delivery_time::ShippingDeliveryTime;
use super::shipping_rate_settings::ShippingRateSettings;
use super::shoe_store::ShoeStore;
use super::shopping_center::ShoppingCenter;
use super::short_story::ShortStory;
use super::side_effects_health_aspect::SideEffectsHealthAspect;
use super::single_blinded_trial::SingleBlindedTrial;
use super::single_center_trial::SingleCenterTrial;
use super::single_family_residence::SingleFamilyResidence;
use super::single_player::SinglePlayer;
use super::single_release::SingleRelease;
use super::site_navigation_element::SiteNavigationElement;
use super::size_group_enumeration::SizeGroupEnumeration;
use super::size_specification::SizeSpecification;
use super::size_system_enumeration::SizeSystemEnumeration;
use super::size_system_imperial::SizeSystemImperial;
use super::size_system_metric::SizeSystemMetric;
use super::ski_resort::SkiResort;
use super::skin::Skin;
use super::social_event::SocialEvent;
use super::social_media_posting::SocialMediaPosting;
use super::software_application::SoftwareApplication;
use super::software_source_code::SoftwareSourceCode;
use super::sold_out::SoldOut;
use super::solve_math_action::SolveMathAction;
use super::some_products::SomeProducts;
use super::soundtrack_album::SoundtrackAlbum;
use super::speakable_specification::SpeakableSpecification;
use super::special_announcement::SpecialAnnouncement;
use super::specialty::Specialty;
use super::speech_pathology::SpeechPathology;
use super::spoken_word_album::SpokenWordAlbum;
use super::sporting_goods_store::SportingGoodsStore;
use super::sports_activity_location::SportsActivityLocation;
use super::sports_club::SportsClub;
use super::sports_event::SportsEvent;
use super::sports_organization::SportsOrganization;
use super::sports_team::SportsTeam;
use super::spreadsheet_digital_document::SpreadsheetDigitalDocument;
use super::stadium_or_arena::StadiumOrArena;
use super::staged_content::StagedContent;
use super::stages_health_aspect::StagesHealthAspect;
use super::state::State;
use super::statement::Statement;
use super::statistical_population::StatisticalPopulation;
use super::status_enumeration::StatusEnumeration;
use super::steering_position_value::SteeringPositionValue;
use super::store::Store;
use super::store_credit_refund::StoreCreditRefund;
use super::strength_training::StrengthTraining;
use super::string::String;
use super::structured_value::StructuredValue;
use super::studio_album::StudioAlbum;
use super::subscribe_action::SubscribeAction;
use super::subscription::Subscription;
use super::substance::Substance;
use super::subway_station::SubwayStation;
use super::suite::Suite;
use super::sunday::Sunday;
use super::superficial_anatomy::SuperficialAnatomy;
use super::surgical::Surgical;
use super::surgical_procedure::SurgicalProcedure;
use super::suspend_action::SuspendAction;
use super::suspended::Suspended;
use super::symptoms_health_aspect::SymptomsHealthAspect;
use super::synagogue::Synagogue;
use super::tv_clip::TVClip;
use super::tv_episode::TVEpisode;
use super::tv_season::TVSeason;
use super::tv_series::TVSeries;
use super::table::Table;
use super::take_action::TakeAction;
use super::tattoo_parlor::TattooParlor;
use super::taxi_reservation::TaxiReservation;
use super::taxi_service::TaxiService;
use super::taxi_stand::TaxiStand;
use super::taxi_vehicle_usage::TaxiVehicleUsage;
use super::taxon::Taxon;
use super::tech_article::TechArticle;
use super::television_channel::TelevisionChannel;
use super::television_station::TelevisionStation;
use super::tennis_complex::TennisComplex;
use super::terminated::Terminated;
use super::text::Text;
use super::text_digital_document::TextDigitalDocument;
use super::theater_event::TheaterEvent;
use super::theater_group::TheaterGroup;
use super::therapeutic::Therapeutic;
use super::therapeutic_procedure::TherapeuticProcedure;
use super::thesis::Thesis;
use super::thing::Thing;
use super::throat::Throat;
use super::thursday::Thursday;
use super::ticket::Ticket;
use super::tie_action::TieAction;
use super::time::Time;
use super::tip_action::TipAction;
use super::tire_shop::TireShop;
use super::tobacco_nicotine_consideration::TobaccoNicotineConsideration;
use super::toll_free::TollFree;
use super::tourist_attraction::TouristAttraction;
use super::tourist_destination::TouristDestination;
use super::tourist_information_center::TouristInformationCenter;
use super::tourist_trip::TouristTrip;
use super::toxicologic::Toxicologic;
use super::toy_store::ToyStore;
use super::track_action::TrackAction;
use super::trade_action::TradeAction;
use super::traditional_chinese::TraditionalChinese;
use super::train_reservation::TrainReservation;
use super::train_station::TrainStation;
use super::train_trip::TrainTrip;
use super::transfer_action::TransferAction;
use super::transformed_content::TransformedContent;
use super::transit_map::TransitMap;
use super::travel_action::TravelAction;
use super::travel_agency::TravelAgency;
use super::treatment_indication::TreatmentIndication;
use super::treatments_health_aspect::TreatmentsHealthAspect;
use super::trip::Trip;
use super::triple_blinded_trial::TripleBlindedTrial;
use super::true::True;
use super::tuesday::Tuesday;
use super::type_and_quantity_node::TypeAndQuantityNode;
use super::types_health_aspect::TypesHealthAspect;
use super::uk_nonprofit_type::UKNonprofitType;
use super::uk_trust::UKTrust;
use super::url::URL;
use super::us_nonprofit_type::USNonprofitType;
use super::ultrasound::Ultrasound;
use super::un_register_action::UnRegisterAction;
use super::unclassified_adult_consideration::UnclassifiedAdultConsideration;
use super::unemployment_support::UnemploymentSupport;
use super::unincorporated_association_charity::UnincorporatedAssociationCharity;
use super::unit_price_specification::UnitPriceSpecification;
use super::unofficial_legal_value::UnofficialLegalValue;
use super::unsigned_integer::UnsignedInteger;
use super::update_action::UpdateAction;
use super::urologic::Urologic;
use super::usage_or_schedule_health_aspect::UsageOrScheduleHealthAspect;
use super::use_action::UseAction;
use super::used_condition::UsedCondition;
use super::user_review::UserReview;
use super::vegan_diet::VeganDiet;
use super::vegetarian_diet::VegetarianDiet;
use super::vehicle::Vehicle;
use super::vein::Vein;
use super::venue_map::VenueMap;
use super::vessel::Vessel;
use super::veterinary_care::VeterinaryCare;
use super::video_gallery::VideoGallery;
use super::video_game::VideoGame;
use super::video_game_clip::VideoGameClip;
use super::video_game_series::VideoGameSeries;
use super::video_object::VideoObject;
use super::video_object_snapshot::VideoObjectSnapshot;
use super::view_action::ViewAction;
use super::vinyl_format::VinylFormat;
use super::violence_consideration::ViolenceConsideration;
use super::virtual_location::VirtualLocation;
use super::virus::Virus;
use super::visual_arts_event::VisualArtsEvent;
use super::visual_artwork::VisualArtwork;
use super::vital_sign::VitalSign;
use super::volcano::Volcano;
use super::vote_action::VoteAction;
use super::wp_ad_block::WPAdBlock;
use super::wp_footer::WPFooter;
use super::wp_header::WPHeader;
use super::wp_side_bar::WPSideBar;
use super::want_action::WantAction;
use super::warranty_promise::WarrantyPromise;
use super::warranty_scope::WarrantyScope;
use super::watch_action::WatchAction;
use super::waterfall::Waterfall;
use super::weapon_consideration::WeaponConsideration;
use super::wear_action::WearAction;
use super::wearable_measurement_back::WearableMeasurementBack;
use super::wearable_measurement_chest_or_bust::WearableMeasurementChestOrBust;
use super::wearable_measurement_collar::WearableMeasurementCollar;
use super::wearable_measurement_cup::WearableMeasurementCup;
use super::wearable_measurement_height::WearableMeasurementHeight;
use super::wearable_measurement_hips::WearableMeasurementHips;
use super::wearable_measurement_inseam::WearableMeasurementInseam;
use super::wearable_measurement_length::WearableMeasurementLength;
use super::wearable_measurement_outside_leg::WearableMeasurementOutsideLeg;
use super::wearable_measurement_sleeve::WearableMeasurementSleeve;
use super::wearable_measurement_type_enumeration::WearableMeasurementTypeEnumeration;
use super::wearable_measurement_waist::WearableMeasurementWaist;
use super::wearable_measurement_width::WearableMeasurementWidth;
use super::wearable_size_group_big::WearableSizeGroupBig;
use super::wearable_size_group_boys::WearableSizeGroupBoys;
use super::wearable_size_group_enumeration::WearableSizeGroupEnumeration;
use super::wearable_size_group_extra_short::WearableSizeGroupExtraShort;
use super::wearable_size_group_extra_tall::WearableSizeGroupExtraTall;
use super::wearable_size_group_girls::WearableSizeGroupGirls;
use super::wearable_size_group_husky::WearableSizeGroupHusky;
use super::wearable_size_group_infants::WearableSizeGroupInfants;
use super::wearable_size_group_juniors::WearableSizeGroupJuniors;
use super::wearable_size_group_maternity::WearableSizeGroupMaternity;
use super::wearable_size_group_mens::WearableSizeGroupMens;
use super::wearable_size_group_misses::WearableSizeGroupMisses;
use super::wearable_size_group_petite::WearableSizeGroupPetite;
use super::wearable_size_group_plus::WearableSizeGroupPlus;
use super::wearable_size_group_regular::WearableSizeGroupRegular;
use super::wearable_size_group_short::WearableSizeGroupShort;
use super::wearable_size_group_tall::WearableSizeGroupTall;
use super::wearable_size_group_womens::WearableSizeGroupWomens;
use super::wearable_size_system_au::WearableSizeSystemAU;
use super::wearable_size_system_br::WearableSizeSystemBR;
use super::wearable_size_system_cn::WearableSizeSystemCN;
use super::wearable_size_system_continental::WearableSizeSystemContinental;
use super::wearable_size_system_de::WearableSizeSystemDE;
use super::wearable_size_system_en13402::WearableSizeSystemEN13402;
use super::wearable_size_system_enumeration::WearableSizeSystemEnumeration;
use super::wearable_size_system_europe::WearableSizeSystemEurope;
use super::wearable_size_system_fr::WearableSizeSystemFR;
use super::wearable_size_system_gs1::WearableSizeSystemGS1;
use super::wearable_size_system_it::WearableSizeSystemIT;
use super::wearable_size_system_jp::WearableSizeSystemJP;
use super::wearable_size_system_mx::WearableSizeSystemMX;
use super::wearable_size_system_uk::WearableSizeSystemUK;
use super::wearable_size_system_us::WearableSizeSystemUS;
use super::web_api::WebAPI;
use super::web_application::WebApplication;
use super::web_content::WebContent;
use super::web_page::WebPage;
use super::web_page_element::WebPageElement;
use super::web_site::WebSite;
use super::wednesday::Wednesday;
use super::western_conventional::WesternConventional;
use super::wholesale::Wholesale;
use super::wholesale_store::WholesaleStore;
use super::win_action::WinAction;
use super::winery::Winery;
use super::withdrawn::Withdrawn;
use super::work_based_program::WorkBasedProgram;
use super::workers_union::WorkersUnion;
use super::write_action::WriteAction;
use super::write_permission::WritePermission;
use super::x_path_type::XPathType;
use super::x_ray::XRay;
use super::zone_boarding_policy::ZoneBoardingPolicy;
use super::zoo::Zoo;

/// Union type for all types in this schema, including primitives and entities
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum Node {
    Null(Null),
    Boolean(Boolean),
    Integer(Integer),
    UnsignedInteger(UnsignedInteger),
    Number(Number),
    String(String),
    Array(Array),
    AMRadioChannel(AMRadioChannel),
    APIReference(APIReference),
    Abdomen(Abdomen),
    AboutPage(AboutPage),
    AcceptAction(AcceptAction),
    Accommodation(Accommodation),
    AccountingService(AccountingService),
    AchieveAction(AchieveAction),
    Action(Action),
    ActionAccessSpecification(ActionAccessSpecification),
    ActionStatusType(ActionStatusType),
    ActivateAction(ActivateAction),
    ActivationFee(ActivationFee),
    ActiveActionStatus(ActiveActionStatus),
    ActiveNotRecruiting(ActiveNotRecruiting),
    AddAction(AddAction),
    AdministrativeArea(AdministrativeArea),
    AdultEntertainment(AdultEntertainment),
    AdultOrientedEnumeration(AdultOrientedEnumeration),
    AdvertiserContentArticle(AdvertiserContentArticle),
    AerobicActivity(AerobicActivity),
    AggregateOffer(AggregateOffer),
    AggregateRating(AggregateRating),
    AgreeAction(AgreeAction),
    Airline(Airline),
    Airport(Airport),
    AlbumRelease(AlbumRelease),
    AlcoholConsideration(AlcoholConsideration),
    AlignmentObject(AlignmentObject),
    AllWheelDriveConfiguration(AllWheelDriveConfiguration),
    AllergiesHealthAspect(AllergiesHealthAspect),
    AllocateAction(AllocateAction),
    AmpStory(AmpStory),
    AmusementPark(AmusementPark),
    AnaerobicActivity(AnaerobicActivity),
    AnalysisNewsArticle(AnalysisNewsArticle),
    AnatomicalStructure(AnatomicalStructure),
    AnatomicalSystem(AnatomicalSystem),
    AndroidPlatform(AndroidPlatform),
    Anesthesia(Anesthesia),
    AnimalShelter(AnimalShelter),
    Answer(Answer),
    Apartment(Apartment),
    ApartmentComplex(ApartmentComplex),
    Appearance(Appearance),
    AppendAction(AppendAction),
    ApplyAction(ApplyAction),
    ApprovedIndication(ApprovedIndication),
    Aquarium(Aquarium),
    ArchiveComponent(ArchiveComponent),
    ArchiveOrganization(ArchiveOrganization),
    ArriveAction(ArriveAction),
    ArtGallery(ArtGallery),
    Artery(Artery),
    Article(Article),
    AskAction(AskAction),
    AskPublicNewsArticle(AskPublicNewsArticle),
    AssessAction(AssessAction),
    AssignAction(AssignAction),
    Atlas(Atlas),
    Attorney(Attorney),
    Audience(Audience),
    AudioObject(AudioObject),
    AudioObjectSnapshot(AudioObjectSnapshot),
    Audiobook(Audiobook),
    AudiobookFormat(AudiobookFormat),
    AuthoritativeLegalValue(AuthoritativeLegalValue),
    AuthorizeAction(AuthorizeAction),
    AutoBodyShop(AutoBodyShop),
    AutoDealer(AutoDealer),
    AutoPartsStore(AutoPartsStore),
    AutoRental(AutoRental),
    AutoRepair(AutoRepair),
    AutoWash(AutoWash),
    AutomatedTeller(AutomatedTeller),
    AutomotiveBusiness(AutomotiveBusiness),
    Ayurvedic(Ayurvedic),
    BackOrder(BackOrder),
    BackgroundNewsArticle(BackgroundNewsArticle),
    Bacteria(Bacteria),
    Bakery(Bakery),
    Balance(Balance),
    BankAccount(BankAccount),
    BankOrCreditUnion(BankOrCreditUnion),
    BarOrPub(BarOrPub),
    Barcode(Barcode),
    BasicIncome(BasicIncome),
    Beach(Beach),
    BeautySalon(BeautySalon),
    BedAndBreakfast(BedAndBreakfast),
    BedDetails(BedDetails),
    BedType(BedType),
    BefriendAction(BefriendAction),
    BenefitsHealthAspect(BenefitsHealthAspect),
    BikeStore(BikeStore),
    BioChemEntity(BioChemEntity),
    Blog(Blog),
    BlogPosting(BlogPosting),
    BloodTest(BloodTest),
    BoardingPolicyType(BoardingPolicyType),
    BoatReservation(BoatReservation),
    BoatTerminal(BoatTerminal),
    BoatTrip(BoatTrip),
    BodyMeasurementArm(BodyMeasurementArm),
    BodyMeasurementBust(BodyMeasurementBust),
    BodyMeasurementChest(BodyMeasurementChest),
    BodyMeasurementFoot(BodyMeasurementFoot),
    BodyMeasurementHand(BodyMeasurementHand),
    BodyMeasurementHead(BodyMeasurementHead),
    BodyMeasurementHeight(BodyMeasurementHeight),
    BodyMeasurementHips(BodyMeasurementHips),
    BodyMeasurementInsideLeg(BodyMeasurementInsideLeg),
    BodyMeasurementNeck(BodyMeasurementNeck),
    BodyMeasurementTypeEnumeration(BodyMeasurementTypeEnumeration),
    BodyMeasurementUnderbust(BodyMeasurementUnderbust),
    BodyMeasurementWaist(BodyMeasurementWaist),
    BodyMeasurementWeight(BodyMeasurementWeight),
    BodyOfWater(BodyOfWater),
    Bone(Bone),
    Book(Book),
    BookFormatType(BookFormatType),
    BookSeries(BookSeries),
    BookStore(BookStore),
    BookmarkAction(BookmarkAction),
    Boolean(Boolean),
    BorrowAction(BorrowAction),
    BowlingAlley(BowlingAlley),
    BrainStructure(BrainStructure),
    Brand(Brand),
    BreadcrumbList(BreadcrumbList),
    Brewery(Brewery),
    Bridge(Bridge),
    BroadcastChannel(BroadcastChannel),
    BroadcastEvent(BroadcastEvent),
    BroadcastFrequencySpecification(BroadcastFrequencySpecification),
    BroadcastRelease(BroadcastRelease),
    BroadcastService(BroadcastService),
    BrokerageAccount(BrokerageAccount),
    BuddhistTemple(BuddhistTemple),
    BusOrCoach(BusOrCoach),
    BusReservation(BusReservation),
    BusStation(BusStation),
    BusStop(BusStop),
    BusTrip(BusTrip),
    BusinessAudience(BusinessAudience),
    BusinessEntityType(BusinessEntityType),
    BusinessEvent(BusinessEvent),
    BusinessFunction(BusinessFunction),
    BusinessSupport(BusinessSupport),
    BuyAction(BuyAction),
    CDCPMDRecord(CDCPMDRecord),
    CDFormat(CDFormat),
    CT(CT),
    CableOrSatelliteService(CableOrSatelliteService),
    CafeOrCoffeeShop(CafeOrCoffeeShop),
    Campground(Campground),
    CampingPitch(CampingPitch),
    Canal(Canal),
    CancelAction(CancelAction),
    Car(Car),
    CarUsageType(CarUsageType),
    Cardiovascular(Cardiovascular),
    CardiovascularExam(CardiovascularExam),
    CaseSeries(CaseSeries),
    Casino(Casino),
    CassetteFormat(CassetteFormat),
    CategoryCode(CategoryCode),
    CategoryCodeSet(CategoryCodeSet),
    CatholicChurch(CatholicChurch),
    CausesHealthAspect(CausesHealthAspect),
    Cemetery(Cemetery),
    Chapter(Chapter),
    CharitableIncorporatedOrganization(CharitableIncorporatedOrganization),
    CheckAction(CheckAction),
    CheckInAction(CheckInAction),
    CheckOutAction(CheckOutAction),
    CheckoutPage(CheckoutPage),
    ChemicalSubstance(ChemicalSubstance),
    ChildCare(ChildCare),
    ChildrensEvent(ChildrensEvent),
    Chiropractic(Chiropractic),
    ChooseAction(ChooseAction),
    Church(Church),
    City(City),
    CityHall(CityHall),
    CivicStructure(CivicStructure),
    Claim(Claim),
    ClaimReview(ClaimReview),
    Class(Class),
    CleaningFee(CleaningFee),
    Clinician(Clinician),
    Clip(Clip),
    ClothingStore(ClothingStore),
    CoOp(CoOp),
    CohortStudy(CohortStudy),
    Collection(Collection),
    CollectionPage(CollectionPage),
    CollegeOrUniversity(CollegeOrUniversity),
    ComedyClub(ComedyClub),
    ComedyEvent(ComedyEvent),
    ComicCoverArt(ComicCoverArt),
    ComicIssue(ComicIssue),
    ComicSeries(ComicSeries),
    ComicStory(ComicStory),
    Comment(Comment),
    CommentAction(CommentAction),
    CommentPermission(CommentPermission),
    CommunicateAction(CommunicateAction),
    CommunityHealth(CommunityHealth),
    CompilationAlbum(CompilationAlbum),
    CompleteDataFeed(CompleteDataFeed),
    Completed(Completed),
    CompletedActionStatus(CompletedActionStatus),
    CompoundPriceSpecification(CompoundPriceSpecification),
    ComputerLanguage(ComputerLanguage),
    ComputerStore(ComputerStore),
    ConfirmAction(ConfirmAction),
    Consortium(Consortium),
    ConsumeAction(ConsumeAction),
    ContactPage(ContactPage),
    ContactPoint(ContactPoint),
    ContactPointOption(ContactPointOption),
    ContagiousnessHealthAspect(ContagiousnessHealthAspect),
    Continent(Continent),
    ControlAction(ControlAction),
    ConvenienceStore(ConvenienceStore),
    Conversation(Conversation),
    CookAction(CookAction),
    Corporation(Corporation),
    CorrectionComment(CorrectionComment),
    Country(Country),
    Course(Course),
    CourseInstance(CourseInstance),
    Courthouse(Courthouse),
    CoverArt(CoverArt),
    CovidTestingFacility(CovidTestingFacility),
    CreateAction(CreateAction),
    CreativeWork(CreativeWork),
    CreativeWorkSeason(CreativeWorkSeason),
    CreativeWorkSeries(CreativeWorkSeries),
    CreditCard(CreditCard),
    Crematorium(Crematorium),
    CriticReview(CriticReview),
    CrossSectional(CrossSectional),
    CssSelectorType(CssSelectorType),
    CurrencyConversionService(CurrencyConversionService),
    DDxElement(DDxElement),
    DJMixAlbum(DJMixAlbum),
    DVDFormat(DVDFormat),
    DamagedCondition(DamagedCondition),
    DanceEvent(DanceEvent),
    DanceGroup(DanceGroup),
    DangerousGoodConsideration(DangerousGoodConsideration),
    DataCatalog(DataCatalog),
    DataDownload(DataDownload),
    DataFeed(DataFeed),
    DataFeedItem(DataFeedItem),
    DataType(DataType),
    Dataset(Dataset),
    Date(Date),
    DateTime(DateTime),
    DayOfWeek(DayOfWeek),
    DaySpa(DaySpa),
    DeactivateAction(DeactivateAction),
    DecontextualizedContent(DecontextualizedContent),
    DefenceEstablishment(DefenceEstablishment),
    DefinedRegion(DefinedRegion),
    DefinedTerm(DefinedTerm),
    DefinedTermSet(DefinedTermSet),
    DefinitiveLegalValue(DefinitiveLegalValue),
    DeleteAction(DeleteAction),
    DeliveryChargeSpecification(DeliveryChargeSpecification),
    DeliveryEvent(DeliveryEvent),
    DeliveryMethod(DeliveryMethod),
    DeliveryTimeSettings(DeliveryTimeSettings),
    Demand(Demand),
    DemoAlbum(DemoAlbum),
    DemoGameAvailability(DemoGameAvailability),
    Dentist(Dentist),
    Dentistry(Dentistry),
    DepartAction(DepartAction),
    DepartmentStore(DepartmentStore),
    DepositAccount(DepositAccount),
    Dermatology(Dermatology),
    DesktopWebPlatform(DesktopWebPlatform),
    DiabeticDiet(DiabeticDiet),
    Diagnostic(Diagnostic),
    DiagnosticLab(DiagnosticLab),
    DiagnosticProcedure(DiagnosticProcedure),
    Diet(Diet),
    DietNutrition(DietNutrition),
    DietarySupplement(DietarySupplement),
    DigitalAudioTapeFormat(DigitalAudioTapeFormat),
    DigitalDocument(DigitalDocument),
    DigitalDocumentPermission(DigitalDocumentPermission),
    DigitalDocumentPermissionType(DigitalDocumentPermissionType),
    DigitalFormat(DigitalFormat),
    DigitalPlatformEnumeration(DigitalPlatformEnumeration),
    DisabilitySupport(DisabilitySupport),
    DisagreeAction(DisagreeAction),
    Discontinued(Discontinued),
    DiscoverAction(DiscoverAction),
    DiscussionForumPosting(DiscussionForumPosting),
    DislikeAction(DislikeAction),
    Distance(Distance),
    DistanceFee(DistanceFee),
    Distillery(Distillery),
    DonateAction(DonateAction),
    DoseSchedule(DoseSchedule),
    DoubleBlindedTrial(DoubleBlindedTrial),
    DownloadAction(DownloadAction),
    Downpayment(Downpayment),
    DrawAction(DrawAction),
    Drawing(Drawing),
    DrinkAction(DrinkAction),
    DriveWheelConfigurationValue(DriveWheelConfigurationValue),
    DrivingSchoolVehicleUsage(DrivingSchoolVehicleUsage),
    Drug(Drug),
    DrugClass(DrugClass),
    DrugCost(DrugCost),
    DrugCostCategory(DrugCostCategory),
    DrugLegalStatus(DrugLegalStatus),
    DrugPregnancyCategory(DrugPregnancyCategory),
    DrugPrescriptionStatus(DrugPrescriptionStatus),
    DrugStrength(DrugStrength),
    DryCleaningOrLaundry(DryCleaningOrLaundry),
    Duration(Duration),
    EBook(EBook),
    EPRelease(EPRelease),
    EUEnergyEfficiencyCategoryA(EUEnergyEfficiencyCategoryA),
    EUEnergyEfficiencyCategoryA1Plus(EUEnergyEfficiencyCategoryA1Plus),
    EUEnergyEfficiencyCategoryA2Plus(EUEnergyEfficiencyCategoryA2Plus),
    EUEnergyEfficiencyCategoryA3Plus(EUEnergyEfficiencyCategoryA3Plus),
    EUEnergyEfficiencyCategoryB(EUEnergyEfficiencyCategoryB),
    EUEnergyEfficiencyCategoryC(EUEnergyEfficiencyCategoryC),
    EUEnergyEfficiencyCategoryD(EUEnergyEfficiencyCategoryD),
    EUEnergyEfficiencyCategoryE(EUEnergyEfficiencyCategoryE),
    EUEnergyEfficiencyCategoryF(EUEnergyEfficiencyCategoryF),
    EUEnergyEfficiencyCategoryG(EUEnergyEfficiencyCategoryG),
    EUEnergyEfficiencyEnumeration(EUEnergyEfficiencyEnumeration),
    Ear(Ear),
    EatAction(EatAction),
    EditedOrCroppedContent(EditedOrCroppedContent),
    EducationEvent(EducationEvent),
    EducationalAudience(EducationalAudience),
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    EducationalOccupationalProgram(EducationalOccupationalProgram),
    EducationalOrganization(EducationalOrganization),
    EffectivenessHealthAspect(EffectivenessHealthAspect),
    Electrician(Electrician),
    ElectronicsStore(ElectronicsStore),
    ElementarySchool(ElementarySchool),
    EmailMessage(EmailMessage),
    Embassy(Embassy),
    Emergency(Emergency),
    EmergencyService(EmergencyService),
    EmployeeRole(EmployeeRole),
    EmployerAggregateRating(EmployerAggregateRating),
    EmployerReview(EmployerReview),
    EmploymentAgency(EmploymentAgency),
    Endocrine(Endocrine),
    EndorseAction(EndorseAction),
    EndorsementRating(EndorsementRating),
    Energy(Energy),
    EnergyConsumptionDetails(EnergyConsumptionDetails),
    EnergyEfficiencyEnumeration(EnergyEfficiencyEnumeration),
    EnergyStarCertified(EnergyStarCertified),
    EnergyStarEnergyEfficiencyEnumeration(EnergyStarEnergyEfficiencyEnumeration),
    EngineSpecification(EngineSpecification),
    EnrollingByInvitation(EnrollingByInvitation),
    EntertainmentBusiness(EntertainmentBusiness),
    EntryPoint(EntryPoint),
    Enumeration(Enumeration),
    Episode(Episode),
    Event(Event),
    EventAttendanceModeEnumeration(EventAttendanceModeEnumeration),
    EventCancelled(EventCancelled),
    EventMovedOnline(EventMovedOnline),
    EventPostponed(EventPostponed),
    EventRescheduled(EventRescheduled),
    EventReservation(EventReservation),
    EventScheduled(EventScheduled),
    EventSeries(EventSeries),
    EventStatusType(EventStatusType),
    EventVenue(EventVenue),
    EvidenceLevelA(EvidenceLevelA),
    EvidenceLevelB(EvidenceLevelB),
    EvidenceLevelC(EvidenceLevelC),
    ExchangeRateSpecification(ExchangeRateSpecification),
    ExchangeRefund(ExchangeRefund),
    ExerciseAction(ExerciseAction),
    ExerciseGym(ExerciseGym),
    ExercisePlan(ExercisePlan),
    ExhibitionEvent(ExhibitionEvent),
    Eye(Eye),
    FAQPage(FAQPage),
    FDAcategoryA(FDAcategoryA),
    FDAcategoryB(FDAcategoryB),
    FDAcategoryC(FDAcategoryC),
    FDAcategoryD(FDAcategoryD),
    FDAcategoryX(FDAcategoryX),
    FDAnotEvaluated(FDAnotEvaluated),
    FMRadioChannel(FMRadioChannel),
    FailedActionStatus(FailedActionStatus),
    False(False),
    FastFoodRestaurant(FastFoodRestaurant),
    Female(Female),
    Festival(Festival),
    FilmAction(FilmAction),
    FinancialProduct(FinancialProduct),
    FinancialService(FinancialService),
    FindAction(FindAction),
    FireStation(FireStation),
    Flexibility(Flexibility),
    Flight(Flight),
    FlightReservation(FlightReservation),
    Float(Float),
    FloorPlan(FloorPlan),
    Florist(Florist),
    FollowAction(FollowAction),
    FoodEstablishment(FoodEstablishment),
    FoodEstablishmentReservation(FoodEstablishmentReservation),
    FoodEvent(FoodEvent),
    FoodService(FoodService),
    FourWheelDriveConfiguration(FourWheelDriveConfiguration),
    FreeReturn(FreeReturn),
    Friday(Friday),
    FrontWheelDriveConfiguration(FrontWheelDriveConfiguration),
    FullGameAvailability(FullGameAvailability),
    FullRefund(FullRefund),
    FundingAgency(FundingAgency),
    FundingScheme(FundingScheme),
    Fungus(Fungus),
    FurnitureStore(FurnitureStore),
    Game(Game),
    GameAvailabilityEnumeration(GameAvailabilityEnumeration),
    GamePlayMode(GamePlayMode),
    GameServer(GameServer),
    GameServerStatus(GameServerStatus),
    GardenStore(GardenStore),
    GasStation(GasStation),
    Gastroenterologic(Gastroenterologic),
    GatedResidenceCommunity(GatedResidenceCommunity),
    GenderType(GenderType),
    Gene(Gene),
    GeneralContractor(GeneralContractor),
    GenericWebPlatform(GenericWebPlatform),
    Genetic(Genetic),
    Genitourinary(Genitourinary),
    GeoCircle(GeoCircle),
    GeoCoordinates(GeoCoordinates),
    GeoShape(GeoShape),
    GeospatialGeometry(GeospatialGeometry),
    Geriatric(Geriatric),
    GettingAccessHealthAspect(GettingAccessHealthAspect),
    GiveAction(GiveAction),
    GlutenFreeDiet(GlutenFreeDiet),
    GolfCourse(GolfCourse),
    GovernmentBenefitsType(GovernmentBenefitsType),
    GovernmentBuilding(GovernmentBuilding),
    GovernmentOffice(GovernmentOffice),
    GovernmentOrganization(GovernmentOrganization),
    GovernmentPermit(GovernmentPermit),
    GovernmentService(GovernmentService),
    Grant(Grant),
    GraphicNovel(GraphicNovel),
    GroceryStore(GroceryStore),
    GroupBoardingPolicy(GroupBoardingPolicy),
    Guide(Guide),
    Gynecologic(Gynecologic),
    HVACBusiness(HVACBusiness),
    Hackathon(Hackathon),
    HairSalon(HairSalon),
    HalalDiet(HalalDiet),
    Hardcover(Hardcover),
    HardwareStore(HardwareStore),
    Head(Head),
    HealthAndBeautyBusiness(HealthAndBeautyBusiness),
    HealthAspectEnumeration(HealthAspectEnumeration),
    HealthCare(HealthCare),
    HealthClub(HealthClub),
    HealthInsurancePlan(HealthInsurancePlan),
    HealthPlanCostSharingSpecification(HealthPlanCostSharingSpecification),
    HealthPlanFormulary(HealthPlanFormulary),
    HealthPlanNetwork(HealthPlanNetwork),
    HealthTopicContent(HealthTopicContent),
    HealthcareConsideration(HealthcareConsideration),
    HearingImpairedSupported(HearingImpairedSupported),
    Hematologic(Hematologic),
    HighSchool(HighSchool),
    HinduDiet(HinduDiet),
    HinduTemple(HinduTemple),
    HobbyShop(HobbyShop),
    HomeAndConstructionBusiness(HomeAndConstructionBusiness),
    HomeGoodsStore(HomeGoodsStore),
    Homeopathic(Homeopathic),
    Hospital(Hospital),
    Hostel(Hostel),
    Hotel(Hotel),
    HotelRoom(HotelRoom),
    House(House),
    HousePainter(HousePainter),
    HowItWorksHealthAspect(HowItWorksHealthAspect),
    HowOrWhereHealthAspect(HowOrWhereHealthAspect),
    HowTo(HowTo),
    HowToDirection(HowToDirection),
    HowToItem(HowToItem),
    HowToSection(HowToSection),
    HowToStep(HowToStep),
    HowToSupply(HowToSupply),
    HowToTip(HowToTip),
    HowToTool(HowToTool),
    HyperToc(HyperToc),
    HyperTocEntry(HyperTocEntry),
    IOSPlatform(IOSPlatform),
    IceCreamShop(IceCreamShop),
    IgnoreAction(IgnoreAction),
    ImageGallery(ImageGallery),
    ImageObject(ImageObject),
    ImageObjectSnapshot(ImageObjectSnapshot),
    ImagingTest(ImagingTest),
    InForce(InForce),
    InStock(InStock),
    InStoreOnly(InStoreOnly),
    IndividualProduct(IndividualProduct),
    Infectious(Infectious),
    InfectiousAgentClass(InfectiousAgentClass),
    InfectiousDisease(InfectiousDisease),
    InformAction(InformAction),
    IngredientsHealthAspect(IngredientsHealthAspect),
    InsertAction(InsertAction),
    InstallAction(InstallAction),
    Installment(Installment),
    InsuranceAgency(InsuranceAgency),
    Intangible(Intangible),
    Integer(Integer),
    InteractAction(InteractAction),
    InteractionCounter(InteractionCounter),
    InternationalTrial(InternationalTrial),
    InternetCafe(InternetCafe),
    InvestmentFund(InvestmentFund),
    InvestmentOrDeposit(InvestmentOrDeposit),
    InviteAction(InviteAction),
    Invoice(Invoice),
    InvoicePrice(InvoicePrice),
    ItemAvailability(ItemAvailability),
    ItemList(ItemList),
    ItemListOrderAscending(ItemListOrderAscending),
    ItemListOrderDescending(ItemListOrderDescending),
    ItemListOrderType(ItemListOrderType),
    ItemListUnordered(ItemListUnordered),
    ItemPage(ItemPage),
    JewelryStore(JewelryStore),
    JobPosting(JobPosting),
    JoinAction(JoinAction),
    Joint(Joint),
    KosherDiet(KosherDiet),
    LaboratoryScience(LaboratoryScience),
    LakeBodyOfWater(LakeBodyOfWater),
    Landform(Landform),
    LandmarksOrHistoricalBuildings(LandmarksOrHistoricalBuildings),
    Language(Language),
    LaserDiscFormat(LaserDiscFormat),
    LearningResource(LearningResource),
    LeaveAction(LeaveAction),
    LeftHandDriving(LeftHandDriving),
    LegalForceStatus(LegalForceStatus),
    LegalService(LegalService),
    LegalValueLevel(LegalValueLevel),
    Legislation(Legislation),
    LegislationObject(LegislationObject),
    LegislativeBuilding(LegislativeBuilding),
    LeisureTimeActivity(LeisureTimeActivity),
    LendAction(LendAction),
    Library(Library),
    LibrarySystem(LibrarySystem),
    LifestyleModification(LifestyleModification),
    Ligament(Ligament),
    LikeAction(LikeAction),
    LimitedAvailability(LimitedAvailability),
    LimitedByGuaranteeCharity(LimitedByGuaranteeCharity),
    LinkRole(LinkRole),
    LiquorStore(LiquorStore),
    ListItem(ListItem),
    ListPrice(ListPrice),
    ListenAction(ListenAction),
    LiteraryEvent(LiteraryEvent),
    LiveAlbum(LiveAlbum),
    LiveBlogPosting(LiveBlogPosting),
    LivingWithHealthAspect(LivingWithHealthAspect),
    LoanOrCredit(LoanOrCredit),
    LocalBusiness(LocalBusiness),
    LocationFeatureSpecification(LocationFeatureSpecification),
    LockerDelivery(LockerDelivery),
    Locksmith(Locksmith),
    LodgingBusiness(LodgingBusiness),
    LodgingReservation(LodgingReservation),
    Longitudinal(Longitudinal),
    LoseAction(LoseAction),
    LowCalorieDiet(LowCalorieDiet),
    LowFatDiet(LowFatDiet),
    LowLactoseDiet(LowLactoseDiet),
    LowSaltDiet(LowSaltDiet),
    Lung(Lung),
    LymphaticVessel(LymphaticVessel),
    MRI(MRI),
    MSRP(MSRP),
    Male(Male),
    Manuscript(Manuscript),
    Map(Map),
    MapCategoryType(MapCategoryType),
    MarryAction(MarryAction),
    Mass(Mass),
    MathSolver(MathSolver),
    MaximumDoseSchedule(MaximumDoseSchedule),
    MayTreatHealthAspect(MayTreatHealthAspect),
    MeasurementTypeEnumeration(MeasurementTypeEnumeration),
    MediaGallery(MediaGallery),
    MediaManipulationRatingEnumeration(MediaManipulationRatingEnumeration),
    MediaObject(MediaObject),
    MediaReview(MediaReview),
    MediaReviewItem(MediaReviewItem),
    MediaSubscription(MediaSubscription),
    MedicalAudience(MedicalAudience),
    MedicalAudienceType(MedicalAudienceType),
    MedicalBusiness(MedicalBusiness),
    MedicalCause(MedicalCause),
    MedicalClinic(MedicalClinic),
    MedicalCode(MedicalCode),
    MedicalCondition(MedicalCondition),
    MedicalConditionStage(MedicalConditionStage),
    MedicalContraindication(MedicalContraindication),
    MedicalDevice(MedicalDevice),
    MedicalDevicePurpose(MedicalDevicePurpose),
    MedicalEntity(MedicalEntity),
    MedicalEnumeration(MedicalEnumeration),
    MedicalEvidenceLevel(MedicalEvidenceLevel),
    MedicalGuideline(MedicalGuideline),
    MedicalGuidelineContraindication(MedicalGuidelineContraindication),
    MedicalGuidelineRecommendation(MedicalGuidelineRecommendation),
    MedicalImagingTechnique(MedicalImagingTechnique),
    MedicalIndication(MedicalIndication),
    MedicalIntangible(MedicalIntangible),
    MedicalObservationalStudy(MedicalObservationalStudy),
    MedicalObservationalStudyDesign(MedicalObservationalStudyDesign),
    MedicalOrganization(MedicalOrganization),
    MedicalProcedure(MedicalProcedure),
    MedicalProcedureType(MedicalProcedureType),
    MedicalResearcher(MedicalResearcher),
    MedicalRiskCalculator(MedicalRiskCalculator),
    MedicalRiskEstimator(MedicalRiskEstimator),
    MedicalRiskFactor(MedicalRiskFactor),
    MedicalRiskScore(MedicalRiskScore),
    MedicalScholarlyArticle(MedicalScholarlyArticle),
    MedicalSign(MedicalSign),
    MedicalSignOrSymptom(MedicalSignOrSymptom),
    MedicalSpecialty(MedicalSpecialty),
    MedicalStudy(MedicalStudy),
    MedicalStudyStatus(MedicalStudyStatus),
    MedicalSymptom(MedicalSymptom),
    MedicalTest(MedicalTest),
    MedicalTestPanel(MedicalTestPanel),
    MedicalTherapy(MedicalTherapy),
    MedicalTrial(MedicalTrial),
    MedicalTrialDesign(MedicalTrialDesign),
    MedicalWebPage(MedicalWebPage),
    MedicineSystem(MedicineSystem),
    MeetingRoom(MeetingRoom),
    MensClothingStore(MensClothingStore),
    Menu(Menu),
    MenuItem(MenuItem),
    MenuSection(MenuSection),
    MerchantReturnEnumeration(MerchantReturnEnumeration),
    MerchantReturnFiniteReturnWindow(MerchantReturnFiniteReturnWindow),
    MerchantReturnNotPermitted(MerchantReturnNotPermitted),
    MerchantReturnPolicy(MerchantReturnPolicy),
    MerchantReturnPolicySeasonalOverride(MerchantReturnPolicySeasonalOverride),
    MerchantReturnUnlimitedWindow(MerchantReturnUnlimitedWindow),
    MerchantReturnUnspecified(MerchantReturnUnspecified),
    Message(Message),
    MiddleSchool(MiddleSchool),
    Midwifery(Midwifery),
    MinimumAdvertisedPrice(MinimumAdvertisedPrice),
    MisconceptionsHealthAspect(MisconceptionsHealthAspect),
    MixedEventAttendanceMode(MixedEventAttendanceMode),
    MixtapeAlbum(MixtapeAlbum),
    MobileApplication(MobileApplication),
    MobilePhoneStore(MobilePhoneStore),
    MobileWebPlatform(MobileWebPlatform),
    Model3D(Model3D),
    MolecularEntity(MolecularEntity),
    Monday(Monday),
    MonetaryAmount(MonetaryAmount),
    MonetaryAmountDistribution(MonetaryAmountDistribution),
    MonetaryGrant(MonetaryGrant),
    MoneyTransfer(MoneyTransfer),
    MortgageLoan(MortgageLoan),
    Mosque(Mosque),
    Motel(Motel),
    Motorcycle(Motorcycle),
    MotorcycleDealer(MotorcycleDealer),
    MotorcycleRepair(MotorcycleRepair),
    MotorizedBicycle(MotorizedBicycle),
    Mountain(Mountain),
    MoveAction(MoveAction),
    Movie(Movie),
    MovieClip(MovieClip),
    MovieRentalStore(MovieRentalStore),
    MovieSeries(MovieSeries),
    MovieTheater(MovieTheater),
    MovingCompany(MovingCompany),
    MultiCenterTrial(MultiCenterTrial),
    MultiPlayer(MultiPlayer),
    MulticellularParasite(MulticellularParasite),
    Muscle(Muscle),
    Musculoskeletal(Musculoskeletal),
    MusculoskeletalExam(MusculoskeletalExam),
    Museum(Museum),
    MusicAlbum(MusicAlbum),
    MusicAlbumProductionType(MusicAlbumProductionType),
    MusicAlbumReleaseType(MusicAlbumReleaseType),
    MusicComposition(MusicComposition),
    MusicEvent(MusicEvent),
    MusicGroup(MusicGroup),
    MusicPlaylist(MusicPlaylist),
    MusicRecording(MusicRecording),
    MusicRelease(MusicRelease),
    MusicReleaseFormatType(MusicReleaseFormatType),
    MusicStore(MusicStore),
    MusicVenue(MusicVenue),
    MusicVideoObject(MusicVideoObject),
    NGO(NGO),
    NLNonprofitType(NLNonprofitType),
    NailSalon(NailSalon),
    NarcoticConsideration(NarcoticConsideration),
    Neck(Neck),
    Nerve(Nerve),
    Neuro(Neuro),
    Neurologic(Neurologic),
    NewCondition(NewCondition),
    NewsArticle(NewsArticle),
    NewsMediaOrganization(NewsMediaOrganization),
    Newspaper(Newspaper),
    NightClub(NightClub),
    NoninvasiveProcedure(NoninvasiveProcedure),
    Nonprofit501a(Nonprofit501a),
    Nonprofit501c1(Nonprofit501c1),
    Nonprofit501c10(Nonprofit501c10),
    Nonprofit501c11(Nonprofit501c11),
    Nonprofit501c12(Nonprofit501c12),
    Nonprofit501c13(Nonprofit501c13),
    Nonprofit501c14(Nonprofit501c14),
    Nonprofit501c15(Nonprofit501c15),
    Nonprofit501c16(Nonprofit501c16),
    Nonprofit501c17(Nonprofit501c17),
    Nonprofit501c18(Nonprofit501c18),
    Nonprofit501c19(Nonprofit501c19),
    Nonprofit501c2(Nonprofit501c2),
    Nonprofit501c20(Nonprofit501c20),
    Nonprofit501c21(Nonprofit501c21),
    Nonprofit501c22(Nonprofit501c22),
    Nonprofit501c23(Nonprofit501c23),
    Nonprofit501c24(Nonprofit501c24),
    Nonprofit501c25(Nonprofit501c25),
    Nonprofit501c26(Nonprofit501c26),
    Nonprofit501c27(Nonprofit501c27),
    Nonprofit501c28(Nonprofit501c28),
    Nonprofit501c3(Nonprofit501c3),
    Nonprofit501c4(Nonprofit501c4),
    Nonprofit501c5(Nonprofit501c5),
    Nonprofit501c6(Nonprofit501c6),
    Nonprofit501c7(Nonprofit501c7),
    Nonprofit501c8(Nonprofit501c8),
    Nonprofit501c9(Nonprofit501c9),
    Nonprofit501d(Nonprofit501d),
    Nonprofit501e(Nonprofit501e),
    Nonprofit501f(Nonprofit501f),
    Nonprofit501k(Nonprofit501k),
    Nonprofit501n(Nonprofit501n),
    Nonprofit501q(Nonprofit501q),
    Nonprofit527(Nonprofit527),
    NonprofitANBI(NonprofitANBI),
    NonprofitSBBI(NonprofitSBBI),
    NonprofitType(NonprofitType),
    Nose(Nose),
    NotInForce(NotInForce),
    NotYetRecruiting(NotYetRecruiting),
    Notary(Notary),
    NoteDigitalDocument(NoteDigitalDocument),
    Number(Number),
    Nursing(Nursing),
    NutritionInformation(NutritionInformation),
    OTC(OTC),
    Observation(Observation),
    Observational(Observational),
    Obstetric(Obstetric),
    Occupation(Occupation),
    OccupationalActivity(OccupationalActivity),
    OccupationalExperienceRequirements(OccupationalExperienceRequirements),
    OccupationalTherapy(OccupationalTherapy),
    OceanBodyOfWater(OceanBodyOfWater),
    Offer(Offer),
    OfferCatalog(OfferCatalog),
    OfferForLease(OfferForLease),
    OfferForPurchase(OfferForPurchase),
    OfferItemCondition(OfferItemCondition),
    OfferShippingDetails(OfferShippingDetails),
    OfficeEquipmentStore(OfficeEquipmentStore),
    OfficialLegalValue(OfficialLegalValue),
    OfflineEventAttendanceMode(OfflineEventAttendanceMode),
    OfflinePermanently(OfflinePermanently),
    OfflineTemporarily(OfflineTemporarily),
    OnDemandEvent(OnDemandEvent),
    OnSitePickup(OnSitePickup),
    Oncologic(Oncologic),
    OneTimePayments(OneTimePayments),
    Online(Online),
    OnlineBusiness(OnlineBusiness),
    OnlineEventAttendanceMode(OnlineEventAttendanceMode),
    OnlineFull(OnlineFull),
    OnlineOnly(OnlineOnly),
    OnlineStore(OnlineStore),
    OpenTrial(OpenTrial),
    OpeningHoursSpecification(OpeningHoursSpecification),
    OpinionNewsArticle(OpinionNewsArticle),
    Optician(Optician),
    Optometric(Optometric),
    Order(Order),
    OrderAction(OrderAction),
    OrderCancelled(OrderCancelled),
    OrderDelivered(OrderDelivered),
    OrderInTransit(OrderInTransit),
    OrderItem(OrderItem),
    OrderPaymentDue(OrderPaymentDue),
    OrderPickupAvailable(OrderPickupAvailable),
    OrderProblem(OrderProblem),
    OrderProcessing(OrderProcessing),
    OrderReturned(OrderReturned),
    OrderStatus(OrderStatus),
    Organization(Organization),
    OrganizationRole(OrganizationRole),
    OrganizeAction(OrganizeAction),
    OriginalMediaContent(OriginalMediaContent),
    OriginalShippingFees(OriginalShippingFees),
    Osteopathic(Osteopathic),
    Otolaryngologic(Otolaryngologic),
    OutOfStock(OutOfStock),
    OutletStore(OutletStore),
    OverviewHealthAspect(OverviewHealthAspect),
    OwnershipInfo(OwnershipInfo),
    PET(PET),
    PaidLeave(PaidLeave),
    PaintAction(PaintAction),
    Painting(Painting),
    PalliativeProcedure(PalliativeProcedure),
    Paperback(Paperback),
    ParcelDelivery(ParcelDelivery),
    ParcelService(ParcelService),
    ParentAudience(ParentAudience),
    ParentalSupport(ParentalSupport),
    Park(Park),
    ParkingFacility(ParkingFacility),
    ParkingMap(ParkingMap),
    PartiallyInForce(PartiallyInForce),
    Pathology(Pathology),
    PathologyTest(PathologyTest),
    Patient(Patient),
    PatientExperienceHealthAspect(PatientExperienceHealthAspect),
    PawnShop(PawnShop),
    PayAction(PayAction),
    PaymentAutomaticallyApplied(PaymentAutomaticallyApplied),
    PaymentCard(PaymentCard),
    PaymentChargeSpecification(PaymentChargeSpecification),
    PaymentComplete(PaymentComplete),
    PaymentDeclined(PaymentDeclined),
    PaymentDue(PaymentDue),
    PaymentMethod(PaymentMethod),
    PaymentPastDue(PaymentPastDue),
    PaymentService(PaymentService),
    PaymentStatusType(PaymentStatusType),
    Pediatric(Pediatric),
    PeopleAudience(PeopleAudience),
    PercutaneousProcedure(PercutaneousProcedure),
    PerformAction(PerformAction),
    PerformanceRole(PerformanceRole),
    PerformingArtsTheater(PerformingArtsTheater),
    PerformingGroup(PerformingGroup),
    Periodical(Periodical),
    Permit(Permit),
    Person(Person),
    PetStore(PetStore),
    Pharmacy(Pharmacy),
    PharmacySpecialty(PharmacySpecialty),
    Photograph(Photograph),
    PhotographAction(PhotographAction),
    PhysicalActivity(PhysicalActivity),
    PhysicalActivityCategory(PhysicalActivityCategory),
    PhysicalExam(PhysicalExam),
    PhysicalTherapy(PhysicalTherapy),
    Physician(Physician),
    Physiotherapy(Physiotherapy),
    Place(Place),
    PlaceOfWorship(PlaceOfWorship),
    PlaceboControlledTrial(PlaceboControlledTrial),
    PlanAction(PlanAction),
    PlasticSurgery(PlasticSurgery),
    Play(Play),
    PlayAction(PlayAction),
    PlayGameAction(PlayGameAction),
    Playground(Playground),
    Plumber(Plumber),
    PodcastEpisode(PodcastEpisode),
    PodcastSeason(PodcastSeason),
    PodcastSeries(PodcastSeries),
    Podiatric(Podiatric),
    PoliceStation(PoliceStation),
    Pond(Pond),
    PostOffice(PostOffice),
    PostalAddress(PostalAddress),
    PostalCodeRangeSpecification(PostalCodeRangeSpecification),
    Poster(Poster),
    PotentialActionStatus(PotentialActionStatus),
    PreOrder(PreOrder),
    PreOrderAction(PreOrderAction),
    PreSale(PreSale),
    PregnancyHealthAspect(PregnancyHealthAspect),
    PrependAction(PrependAction),
    Preschool(Preschool),
    PrescriptionOnly(PrescriptionOnly),
    PresentationDigitalDocument(PresentationDigitalDocument),
    PreventionHealthAspect(PreventionHealthAspect),
    PreventionIndication(PreventionIndication),
    PriceComponentTypeEnumeration(PriceComponentTypeEnumeration),
    PriceSpecification(PriceSpecification),
    PriceTypeEnumeration(PriceTypeEnumeration),
    PrimaryCare(PrimaryCare),
    Prion(Prion),
    Product(Product),
    ProductCollection(ProductCollection),
    ProductGroup(ProductGroup),
    ProductModel(ProductModel),
    ProfessionalService(ProfessionalService),
    ProfilePage(ProfilePage),
    PrognosisHealthAspect(PrognosisHealthAspect),
    ProgramMembership(ProgramMembership),
    Project(Project),
    PronounceableText(PronounceableText),
    Property(Property),
    PropertyValue(PropertyValue),
    PropertyValueSpecification(PropertyValueSpecification),
    Protein(Protein),
    Protozoa(Protozoa),
    Psychiatric(Psychiatric),
    PsychologicalTreatment(PsychologicalTreatment),
    PublicHealth(PublicHealth),
    PublicHolidays(PublicHolidays),
    PublicSwimmingPool(PublicSwimmingPool),
    PublicToilet(PublicToilet),
    PublicationEvent(PublicationEvent),
    PublicationIssue(PublicationIssue),
    PublicationVolume(PublicationVolume),
    Pulmonary(Pulmonary),
    QAPage(QAPage),
    QualitativeValue(QualitativeValue),
    QuantitativeValue(QuantitativeValue),
    QuantitativeValueDistribution(QuantitativeValueDistribution),
    Quantity(Quantity),
    Question(Question),
    Quiz(Quiz),
    Quotation(Quotation),
    QuoteAction(QuoteAction),
    RVPark(RVPark),
    RadiationTherapy(RadiationTherapy),
    RadioBroadcastService(RadioBroadcastService),
    RadioChannel(RadioChannel),
    RadioClip(RadioClip),
    RadioEpisode(RadioEpisode),
    RadioSeason(RadioSeason),
    RadioSeries(RadioSeries),
    RadioStation(RadioStation),
    Radiography(Radiography),
    RandomizedTrial(RandomizedTrial),
    Rating(Rating),
    ReactAction(ReactAction),
    ReadAction(ReadAction),
    ReadPermission(ReadPermission),
    RealEstateAgent(RealEstateAgent),
    RealEstateListing(RealEstateListing),
    RearWheelDriveConfiguration(RearWheelDriveConfiguration),
    ReceiveAction(ReceiveAction),
    Recipe(Recipe),
    Recommendation(Recommendation),
    RecommendedDoseSchedule(RecommendedDoseSchedule),
    Recruiting(Recruiting),
    RecyclingCenter(RecyclingCenter),
    ReducedRelevanceForChildrenConsideration(ReducedRelevanceForChildrenConsideration),
    RefundTypeEnumeration(RefundTypeEnumeration),
    RefurbishedCondition(RefurbishedCondition),
    RegisterAction(RegisterAction),
    Registry(Registry),
    ReimbursementCap(ReimbursementCap),
    RejectAction(RejectAction),
    RelatedTopicsHealthAspect(RelatedTopicsHealthAspect),
    RemixAlbum(RemixAlbum),
    Renal(Renal),
    RentAction(RentAction),
    RentalCarReservation(RentalCarReservation),
    RentalVehicleUsage(RentalVehicleUsage),
    RepaymentSpecification(RepaymentSpecification),
    ReplaceAction(ReplaceAction),
    ReplyAction(ReplyAction),
    Report(Report),
    ReportageNewsArticle(ReportageNewsArticle),
    ReportedDoseSchedule(ReportedDoseSchedule),
    ResearchOrganization(ResearchOrganization),
    ResearchProject(ResearchProject),
    Researcher(Researcher),
    Reservation(Reservation),
    ReservationCancelled(ReservationCancelled),
    ReservationConfirmed(ReservationConfirmed),
    ReservationHold(ReservationHold),
    ReservationPackage(ReservationPackage),
    ReservationPending(ReservationPending),
    ReservationStatusType(ReservationStatusType),
    ReserveAction(ReserveAction),
    Reservoir(Reservoir),
    Residence(Residence),
    Resort(Resort),
    RespiratoryTherapy(RespiratoryTherapy),
    Restaurant(Restaurant),
    RestockingFees(RestockingFees),
    RestrictedDiet(RestrictedDiet),
    ResultsAvailable(ResultsAvailable),
    ResultsNotAvailable(ResultsNotAvailable),
    ResumeAction(ResumeAction),
    Retail(Retail),
    ReturnAction(ReturnAction),
    ReturnAtKiosk(ReturnAtKiosk),
    ReturnByMail(ReturnByMail),
    ReturnFeesCustomerResponsibility(ReturnFeesCustomerResponsibility),
    ReturnFeesEnumeration(ReturnFeesEnumeration),
    ReturnInStore(ReturnInStore),
    ReturnLabelCustomerResponsibility(ReturnLabelCustomerResponsibility),
    ReturnLabelDownloadAndPrint(ReturnLabelDownloadAndPrint),
    ReturnLabelInBox(ReturnLabelInBox),
    ReturnLabelSourceEnumeration(ReturnLabelSourceEnumeration),
    ReturnMethodEnumeration(ReturnMethodEnumeration),
    ReturnShippingFees(ReturnShippingFees),
    Review(Review),
    ReviewAction(ReviewAction),
    ReviewNewsArticle(ReviewNewsArticle),
    Rheumatologic(Rheumatologic),
    RightHandDriving(RightHandDriving),
    RisksOrComplicationsHealthAspect(RisksOrComplicationsHealthAspect),
    RiverBodyOfWater(RiverBodyOfWater),
    Role(Role),
    RoofingContractor(RoofingContractor),
    Room(Room),
    RsvpAction(RsvpAction),
    RsvpResponseMaybe(RsvpResponseMaybe),
    RsvpResponseNo(RsvpResponseNo),
    RsvpResponseType(RsvpResponseType),
    RsvpResponseYes(RsvpResponseYes),
    SRP(SRP),
    SafetyHealthAspect(SafetyHealthAspect),
    SaleEvent(SaleEvent),
    SalePrice(SalePrice),
    SatireOrParodyContent(SatireOrParodyContent),
    SatiricalArticle(SatiricalArticle),
    Saturday(Saturday),
    Schedule(Schedule),
    ScheduleAction(ScheduleAction),
    ScholarlyArticle(ScholarlyArticle),
    School(School),
    SchoolDistrict(SchoolDistrict),
    ScreeningEvent(ScreeningEvent),
    ScreeningHealthAspect(ScreeningHealthAspect),
    Sculpture(Sculpture),
    SeaBodyOfWater(SeaBodyOfWater),
    SearchAction(SearchAction),
    SearchRescueOrganization(SearchRescueOrganization),
    SearchResultsPage(SearchResultsPage),
    Seat(Seat),
    SeatingMap(SeatingMap),
    SeeDoctorHealthAspect(SeeDoctorHealthAspect),
    SeekToAction(SeekToAction),
    SelfCareHealthAspect(SelfCareHealthAspect),
    SelfStorage(SelfStorage),
    SellAction(SellAction),
    SendAction(SendAction),
    Series(Series),
    Service(Service),
    ServiceChannel(ServiceChannel),
    SexualContentConsideration(SexualContentConsideration),
    ShareAction(ShareAction),
    SheetMusic(SheetMusic),
    ShippingDeliveryTime(ShippingDeliveryTime),
    ShippingRateSettings(ShippingRateSettings),
    ShoeStore(ShoeStore),
    ShoppingCenter(ShoppingCenter),
    ShortStory(ShortStory),
    SideEffectsHealthAspect(SideEffectsHealthAspect),
    SingleBlindedTrial(SingleBlindedTrial),
    SingleCenterTrial(SingleCenterTrial),
    SingleFamilyResidence(SingleFamilyResidence),
    SinglePlayer(SinglePlayer),
    SingleRelease(SingleRelease),
    SiteNavigationElement(SiteNavigationElement),
    SizeGroupEnumeration(SizeGroupEnumeration),
    SizeSpecification(SizeSpecification),
    SizeSystemEnumeration(SizeSystemEnumeration),
    SizeSystemImperial(SizeSystemImperial),
    SizeSystemMetric(SizeSystemMetric),
    SkiResort(SkiResort),
    Skin(Skin),
    SocialEvent(SocialEvent),
    SocialMediaPosting(SocialMediaPosting),
    SoftwareApplication(SoftwareApplication),
    SoftwareSourceCode(SoftwareSourceCode),
    SoldOut(SoldOut),
    SolveMathAction(SolveMathAction),
    SomeProducts(SomeProducts),
    SoundtrackAlbum(SoundtrackAlbum),
    SpeakableSpecification(SpeakableSpecification),
    SpecialAnnouncement(SpecialAnnouncement),
    Specialty(Specialty),
    SpeechPathology(SpeechPathology),
    SpokenWordAlbum(SpokenWordAlbum),
    SportingGoodsStore(SportingGoodsStore),
    SportsActivityLocation(SportsActivityLocation),
    SportsClub(SportsClub),
    SportsEvent(SportsEvent),
    SportsOrganization(SportsOrganization),
    SportsTeam(SportsTeam),
    SpreadsheetDigitalDocument(SpreadsheetDigitalDocument),
    StadiumOrArena(StadiumOrArena),
    StagedContent(StagedContent),
    StagesHealthAspect(StagesHealthAspect),
    State(State),
    Statement(Statement),
    StatisticalPopulation(StatisticalPopulation),
    StatusEnumeration(StatusEnumeration),
    SteeringPositionValue(SteeringPositionValue),
    Store(Store),
    StoreCreditRefund(StoreCreditRefund),
    StrengthTraining(StrengthTraining),
    StructuredValue(StructuredValue),
    StudioAlbum(StudioAlbum),
    SubscribeAction(SubscribeAction),
    Subscription(Subscription),
    Substance(Substance),
    SubwayStation(SubwayStation),
    Suite(Suite),
    Sunday(Sunday),
    SuperficialAnatomy(SuperficialAnatomy),
    Surgical(Surgical),
    SurgicalProcedure(SurgicalProcedure),
    SuspendAction(SuspendAction),
    Suspended(Suspended),
    SymptomsHealthAspect(SymptomsHealthAspect),
    Synagogue(Synagogue),
    TVClip(TVClip),
    TVEpisode(TVEpisode),
    TVSeason(TVSeason),
    TVSeries(TVSeries),
    Table(Table),
    TakeAction(TakeAction),
    TattooParlor(TattooParlor),
    TaxiReservation(TaxiReservation),
    TaxiService(TaxiService),
    TaxiStand(TaxiStand),
    TaxiVehicleUsage(TaxiVehicleUsage),
    Taxon(Taxon),
    TechArticle(TechArticle),
    TelevisionChannel(TelevisionChannel),
    TelevisionStation(TelevisionStation),
    TennisComplex(TennisComplex),
    Terminated(Terminated),
    Text(Text),
    TextDigitalDocument(TextDigitalDocument),
    TheaterEvent(TheaterEvent),
    TheaterGroup(TheaterGroup),
    Therapeutic(Therapeutic),
    TherapeuticProcedure(TherapeuticProcedure),
    Thesis(Thesis),
    Thing(Thing),
    Throat(Throat),
    Thursday(Thursday),
    Ticket(Ticket),
    TieAction(TieAction),
    Time(Time),
    TipAction(TipAction),
    TireShop(TireShop),
    TobaccoNicotineConsideration(TobaccoNicotineConsideration),
    TollFree(TollFree),
    TouristAttraction(TouristAttraction),
    TouristDestination(TouristDestination),
    TouristInformationCenter(TouristInformationCenter),
    TouristTrip(TouristTrip),
    Toxicologic(Toxicologic),
    ToyStore(ToyStore),
    TrackAction(TrackAction),
    TradeAction(TradeAction),
    TraditionalChinese(TraditionalChinese),
    TrainReservation(TrainReservation),
    TrainStation(TrainStation),
    TrainTrip(TrainTrip),
    TransferAction(TransferAction),
    TransformedContent(TransformedContent),
    TransitMap(TransitMap),
    TravelAction(TravelAction),
    TravelAgency(TravelAgency),
    TreatmentIndication(TreatmentIndication),
    TreatmentsHealthAspect(TreatmentsHealthAspect),
    Trip(Trip),
    TripleBlindedTrial(TripleBlindedTrial),
    True(True),
    Tuesday(Tuesday),
    TypeAndQuantityNode(TypeAndQuantityNode),
    TypesHealthAspect(TypesHealthAspect),
    UKNonprofitType(UKNonprofitType),
    UKTrust(UKTrust),
    URL(URL),
    USNonprofitType(USNonprofitType),
    Ultrasound(Ultrasound),
    UnRegisterAction(UnRegisterAction),
    UnclassifiedAdultConsideration(UnclassifiedAdultConsideration),
    UnemploymentSupport(UnemploymentSupport),
    UnincorporatedAssociationCharity(UnincorporatedAssociationCharity),
    UnitPriceSpecification(UnitPriceSpecification),
    UnofficialLegalValue(UnofficialLegalValue),
    UpdateAction(UpdateAction),
    Urologic(Urologic),
    UsageOrScheduleHealthAspect(UsageOrScheduleHealthAspect),
    UseAction(UseAction),
    UsedCondition(UsedCondition),
    UserReview(UserReview),
    VeganDiet(VeganDiet),
    VegetarianDiet(VegetarianDiet),
    Vehicle(Vehicle),
    Vein(Vein),
    VenueMap(VenueMap),
    Vessel(Vessel),
    VeterinaryCare(VeterinaryCare),
    VideoGallery(VideoGallery),
    VideoGame(VideoGame),
    VideoGameClip(VideoGameClip),
    VideoGameSeries(VideoGameSeries),
    VideoObject(VideoObject),
    VideoObjectSnapshot(VideoObjectSnapshot),
    ViewAction(ViewAction),
    VinylFormat(VinylFormat),
    ViolenceConsideration(ViolenceConsideration),
    VirtualLocation(VirtualLocation),
    Virus(Virus),
    VisualArtsEvent(VisualArtsEvent),
    VisualArtwork(VisualArtwork),
    VitalSign(VitalSign),
    Volcano(Volcano),
    VoteAction(VoteAction),
    WPAdBlock(WPAdBlock),
    WPFooter(WPFooter),
    WPHeader(WPHeader),
    WPSideBar(WPSideBar),
    WantAction(WantAction),
    WarrantyPromise(WarrantyPromise),
    WarrantyScope(WarrantyScope),
    WatchAction(WatchAction),
    Waterfall(Waterfall),
    WeaponConsideration(WeaponConsideration),
    WearAction(WearAction),
    WearableMeasurementBack(WearableMeasurementBack),
    WearableMeasurementChestOrBust(WearableMeasurementChestOrBust),
    WearableMeasurementCollar(WearableMeasurementCollar),
    WearableMeasurementCup(WearableMeasurementCup),
    WearableMeasurementHeight(WearableMeasurementHeight),
    WearableMeasurementHips(WearableMeasurementHips),
    WearableMeasurementInseam(WearableMeasurementInseam),
    WearableMeasurementLength(WearableMeasurementLength),
    WearableMeasurementOutsideLeg(WearableMeasurementOutsideLeg),
    WearableMeasurementSleeve(WearableMeasurementSleeve),
    WearableMeasurementTypeEnumeration(WearableMeasurementTypeEnumeration),
    WearableMeasurementWaist(WearableMeasurementWaist),
    WearableMeasurementWidth(WearableMeasurementWidth),
    WearableSizeGroupBig(WearableSizeGroupBig),
    WearableSizeGroupBoys(WearableSizeGroupBoys),
    WearableSizeGroupEnumeration(WearableSizeGroupEnumeration),
    WearableSizeGroupExtraShort(WearableSizeGroupExtraShort),
    WearableSizeGroupExtraTall(WearableSizeGroupExtraTall),
    WearableSizeGroupGirls(WearableSizeGroupGirls),
    WearableSizeGroupHusky(WearableSizeGroupHusky),
    WearableSizeGroupInfants(WearableSizeGroupInfants),
    WearableSizeGroupJuniors(WearableSizeGroupJuniors),
    WearableSizeGroupMaternity(WearableSizeGroupMaternity),
    WearableSizeGroupMens(WearableSizeGroupMens),
    WearableSizeGroupMisses(WearableSizeGroupMisses),
    WearableSizeGroupPetite(WearableSizeGroupPetite),
    WearableSizeGroupPlus(WearableSizeGroupPlus),
    WearableSizeGroupRegular(WearableSizeGroupRegular),
    WearableSizeGroupShort(WearableSizeGroupShort),
    WearableSizeGroupTall(WearableSizeGroupTall),
    WearableSizeGroupWomens(WearableSizeGroupWomens),
    WearableSizeSystemAU(WearableSizeSystemAU),
    WearableSizeSystemBR(WearableSizeSystemBR),
    WearableSizeSystemCN(WearableSizeSystemCN),
    WearableSizeSystemContinental(WearableSizeSystemContinental),
    WearableSizeSystemDE(WearableSizeSystemDE),
    WearableSizeSystemEN13402(WearableSizeSystemEN13402),
    WearableSizeSystemEnumeration(WearableSizeSystemEnumeration),
    WearableSizeSystemEurope(WearableSizeSystemEurope),
    WearableSizeSystemFR(WearableSizeSystemFR),
    WearableSizeSystemGS1(WearableSizeSystemGS1),
    WearableSizeSystemIT(WearableSizeSystemIT),
    WearableSizeSystemJP(WearableSizeSystemJP),
    WearableSizeSystemMX(WearableSizeSystemMX),
    WearableSizeSystemUK(WearableSizeSystemUK),
    WearableSizeSystemUS(WearableSizeSystemUS),
    WebAPI(WebAPI),
    WebApplication(WebApplication),
    WebContent(WebContent),
    WebPage(WebPage),
    WebPageElement(WebPageElement),
    WebSite(WebSite),
    Wednesday(Wednesday),
    WesternConventional(WesternConventional),
    Wholesale(Wholesale),
    WholesaleStore(WholesaleStore),
    WinAction(WinAction),
    Winery(Winery),
    Withdrawn(Withdrawn),
    WorkBasedProgram(WorkBasedProgram),
    WorkersUnion(WorkersUnion),
    WriteAction(WriteAction),
    WritePermission(WritePermission),
    XPathType(XPathType),
    XRay(XRay),
    ZoneBoardingPolicy(ZoneBoardingPolicy),
    Zoo(Zoo),
    Object(Object),
}
