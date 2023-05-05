use crate::prelude::*;

use super::api_reference::APIReference;
use super::about_page::AboutPage;
use super::advertiser_content_article::AdvertiserContentArticle;
use super::amp_story::AmpStory;
use super::analysis_news_article::AnalysisNewsArticle;
use super::answer::Answer;
use super::archive_component::ArchiveComponent;
use super::article::Article;
use super::ask_public_news_article::AskPublicNewsArticle;
use super::atlas::Atlas;
use super::audio_object::AudioObject;
use super::audio_object_snapshot::AudioObjectSnapshot;
use super::audiobook::Audiobook;
use super::background_news_article::BackgroundNewsArticle;
use super::barcode::Barcode;
use super::blog::Blog;
use super::blog_posting::BlogPosting;
use super::book::Book;
use super::book_series::BookSeries;
use super::category_code_set::CategoryCodeSet;
use super::chapter::Chapter;
use super::checkout_page::CheckoutPage;
use super::claim::Claim;
use super::claim_review::ClaimReview;
use super::clip::Clip;
use super::collection::Collection;
use super::collection_page::CollectionPage;
use super::comic_cover_art::ComicCoverArt;
use super::comic_issue::ComicIssue;
use super::comic_series::ComicSeries;
use super::comic_story::ComicStory;
use super::comment::Comment;
use super::complete_data_feed::CompleteDataFeed;
use super::contact_page::ContactPage;
use super::conversation::Conversation;
use super::correction_comment::CorrectionComment;
use super::course::Course;
use super::cover_art::CoverArt;
use super::creative_work_season::CreativeWorkSeason;
use super::creative_work_series::CreativeWorkSeries;
use super::critic_review::CriticReview;
use super::data_catalog::DataCatalog;
use super::data_download::DataDownload;
use super::data_feed::DataFeed;
use super::dataset::Dataset;
use super::defined_term_set::DefinedTermSet;
use super::diet::Diet;
use super::digital_document::DigitalDocument;
use super::discussion_forum_posting::DiscussionForumPosting;
use super::drawing::Drawing;
use super::educational_occupational_credential::EducationalOccupationalCredential;
use super::email_message::EmailMessage;
use super::employer_review::EmployerReview;
use super::episode::Episode;
use super::exercise_plan::ExercisePlan;
use super::faq_page::FAQPage;
use super::game::Game;
use super::guide::Guide;
use super::health_topic_content::HealthTopicContent;
use super::how_to::HowTo;
use super::how_to_direction::HowToDirection;
use super::how_to_section::HowToSection;
use super::how_to_step::HowToStep;
use super::how_to_tip::HowToTip;
use super::hyper_toc::HyperToc;
use super::hyper_toc_entry::HyperTocEntry;
use super::image_gallery::ImageGallery;
use super::image_object::ImageObject;
use super::image_object_snapshot::ImageObjectSnapshot;
use super::item_page::ItemPage;
use super::learning_resource::LearningResource;
use super::legislation::Legislation;
use super::legislation_object::LegislationObject;
use super::live_blog_posting::LiveBlogPosting;
use super::manuscript::Manuscript;
use super::map::Map;
use super::math_solver::MathSolver;
use super::media_gallery::MediaGallery;
use super::media_object::MediaObject;
use super::media_review::MediaReview;
use super::media_review_item::MediaReviewItem;
use super::medical_scholarly_article::MedicalScholarlyArticle;
use super::medical_web_page::MedicalWebPage;
use super::menu::Menu;
use super::menu_section::MenuSection;
use super::message::Message;
use super::mobile_application::MobileApplication;
use super::model_3d::Model3D;
use super::movie::Movie;
use super::movie_clip::MovieClip;
use super::movie_series::MovieSeries;
use super::music_album::MusicAlbum;
use super::music_composition::MusicComposition;
use super::music_playlist::MusicPlaylist;
use super::music_recording::MusicRecording;
use super::music_release::MusicRelease;
use super::music_video_object::MusicVideoObject;
use super::news_article::NewsArticle;
use super::newspaper::Newspaper;
use super::note_digital_document::NoteDigitalDocument;
use super::opinion_news_article::OpinionNewsArticle;
use super::painting::Painting;
use super::periodical::Periodical;
use super::photograph::Photograph;
use super::play::Play;
use super::podcast_episode::PodcastEpisode;
use super::podcast_season::PodcastSeason;
use super::podcast_series::PodcastSeries;
use super::poster::Poster;
use super::presentation_digital_document::PresentationDigitalDocument;
use super::product_collection::ProductCollection;
use super::profile_page::ProfilePage;
use super::publication_issue::PublicationIssue;
use super::publication_volume::PublicationVolume;
use super::qa_page::QAPage;
use super::question::Question;
use super::quiz::Quiz;
use super::quotation::Quotation;
use super::radio_clip::RadioClip;
use super::radio_episode::RadioEpisode;
use super::radio_season::RadioSeason;
use super::radio_series::RadioSeries;
use super::real_estate_listing::RealEstateListing;
use super::recipe::Recipe;
use super::recommendation::Recommendation;
use super::report::Report;
use super::reportage_news_article::ReportageNewsArticle;
use super::review::Review;
use super::review_news_article::ReviewNewsArticle;
use super::satirical_article::SatiricalArticle;
use super::scholarly_article::ScholarlyArticle;
use super::sculpture::Sculpture;
use super::search_results_page::SearchResultsPage;
use super::sheet_music::SheetMusic;
use super::short_story::ShortStory;
use super::site_navigation_element::SiteNavigationElement;
use super::social_media_posting::SocialMediaPosting;
use super::software_application::SoftwareApplication;
use super::software_source_code::SoftwareSourceCode;
use super::special_announcement::SpecialAnnouncement;
use super::spreadsheet_digital_document::SpreadsheetDigitalDocument;
use super::statement::Statement;
use super::tv_clip::TVClip;
use super::tv_episode::TVEpisode;
use super::tv_season::TVSeason;
use super::tv_series::TVSeries;
use super::table::Table;
use super::tech_article::TechArticle;
use super::text_digital_document::TextDigitalDocument;
use super::thesis::Thesis;
use super::user_review::UserReview;
use super::video_gallery::VideoGallery;
use super::video_game::VideoGame;
use super::video_game_clip::VideoGameClip;
use super::video_game_series::VideoGameSeries;
use super::video_object::VideoObject;
use super::video_object_snapshot::VideoObjectSnapshot;
use super::visual_artwork::VisualArtwork;
use super::wp_ad_block::WPAdBlock;
use super::wp_footer::WPFooter;
use super::wp_header::WPHeader;
use super::wp_side_bar::WPSideBar;
use super::web_application::WebApplication;
use super::web_content::WebContent;
use super::web_page::WebPage;
use super::web_page_element::WebPageElement;
use super::web_site::WebSite;

/// Union type for all types that are descended from `CreativeWork`
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum CreativeWorkType {
    APIReference(APIReference),
    AboutPage(AboutPage),
    AdvertiserContentArticle(AdvertiserContentArticle),
    AmpStory(AmpStory),
    AnalysisNewsArticle(AnalysisNewsArticle),
    Answer(Answer),
    ArchiveComponent(ArchiveComponent),
    Article(Article),
    AskPublicNewsArticle(AskPublicNewsArticle),
    Atlas(Atlas),
    AudioObject(AudioObject),
    AudioObjectSnapshot(AudioObjectSnapshot),
    Audiobook(Audiobook),
    BackgroundNewsArticle(BackgroundNewsArticle),
    Barcode(Barcode),
    Blog(Blog),
    BlogPosting(BlogPosting),
    Book(Book),
    BookSeries(BookSeries),
    CategoryCodeSet(CategoryCodeSet),
    Chapter(Chapter),
    CheckoutPage(CheckoutPage),
    Claim(Claim),
    ClaimReview(ClaimReview),
    Clip(Clip),
    Collection(Collection),
    CollectionPage(CollectionPage),
    ComicCoverArt(ComicCoverArt),
    ComicIssue(ComicIssue),
    ComicSeries(ComicSeries),
    ComicStory(ComicStory),
    Comment(Comment),
    CompleteDataFeed(CompleteDataFeed),
    ContactPage(ContactPage),
    Conversation(Conversation),
    CorrectionComment(CorrectionComment),
    Course(Course),
    CoverArt(CoverArt),
    CreativeWorkSeason(CreativeWorkSeason),
    CreativeWorkSeries(CreativeWorkSeries),
    CriticReview(CriticReview),
    DataCatalog(DataCatalog),
    DataDownload(DataDownload),
    DataFeed(DataFeed),
    Dataset(Dataset),
    DefinedTermSet(DefinedTermSet),
    Diet(Diet),
    DigitalDocument(DigitalDocument),
    DiscussionForumPosting(DiscussionForumPosting),
    Drawing(Drawing),
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    EmailMessage(EmailMessage),
    EmployerReview(EmployerReview),
    Episode(Episode),
    ExercisePlan(ExercisePlan),
    FAQPage(FAQPage),
    Game(Game),
    Guide(Guide),
    HealthTopicContent(HealthTopicContent),
    HowTo(HowTo),
    HowToDirection(HowToDirection),
    HowToSection(HowToSection),
    HowToStep(HowToStep),
    HowToTip(HowToTip),
    HyperToc(HyperToc),
    HyperTocEntry(HyperTocEntry),
    ImageGallery(ImageGallery),
    ImageObject(ImageObject),
    ImageObjectSnapshot(ImageObjectSnapshot),
    ItemPage(ItemPage),
    LearningResource(LearningResource),
    Legislation(Legislation),
    LegislationObject(LegislationObject),
    LiveBlogPosting(LiveBlogPosting),
    Manuscript(Manuscript),
    Map(Map),
    MathSolver(MathSolver),
    MediaGallery(MediaGallery),
    MediaObject(MediaObject),
    MediaReview(MediaReview),
    MediaReviewItem(MediaReviewItem),
    MedicalScholarlyArticle(MedicalScholarlyArticle),
    MedicalWebPage(MedicalWebPage),
    Menu(Menu),
    MenuSection(MenuSection),
    Message(Message),
    MobileApplication(MobileApplication),
    Model3D(Model3D),
    Movie(Movie),
    MovieClip(MovieClip),
    MovieSeries(MovieSeries),
    MusicAlbum(MusicAlbum),
    MusicComposition(MusicComposition),
    MusicPlaylist(MusicPlaylist),
    MusicRecording(MusicRecording),
    MusicRelease(MusicRelease),
    MusicVideoObject(MusicVideoObject),
    NewsArticle(NewsArticle),
    Newspaper(Newspaper),
    NoteDigitalDocument(NoteDigitalDocument),
    OpinionNewsArticle(OpinionNewsArticle),
    Painting(Painting),
    Periodical(Periodical),
    Photograph(Photograph),
    Play(Play),
    PodcastEpisode(PodcastEpisode),
    PodcastSeason(PodcastSeason),
    PodcastSeries(PodcastSeries),
    Poster(Poster),
    PresentationDigitalDocument(PresentationDigitalDocument),
    ProductCollection(ProductCollection),
    ProfilePage(ProfilePage),
    PublicationIssue(PublicationIssue),
    PublicationVolume(PublicationVolume),
    QAPage(QAPage),
    Question(Question),
    Quiz(Quiz),
    Quotation(Quotation),
    RadioClip(RadioClip),
    RadioEpisode(RadioEpisode),
    RadioSeason(RadioSeason),
    RadioSeries(RadioSeries),
    RealEstateListing(RealEstateListing),
    Recipe(Recipe),
    Recommendation(Recommendation),
    Report(Report),
    ReportageNewsArticle(ReportageNewsArticle),
    Review(Review),
    ReviewNewsArticle(ReviewNewsArticle),
    SatiricalArticle(SatiricalArticle),
    ScholarlyArticle(ScholarlyArticle),
    Sculpture(Sculpture),
    SearchResultsPage(SearchResultsPage),
    SheetMusic(SheetMusic),
    ShortStory(ShortStory),
    SiteNavigationElement(SiteNavigationElement),
    SocialMediaPosting(SocialMediaPosting),
    SoftwareApplication(SoftwareApplication),
    SoftwareSourceCode(SoftwareSourceCode),
    SpecialAnnouncement(SpecialAnnouncement),
    SpreadsheetDigitalDocument(SpreadsheetDigitalDocument),
    Statement(Statement),
    TVClip(TVClip),
    TVEpisode(TVEpisode),
    TVSeason(TVSeason),
    TVSeries(TVSeries),
    Table(Table),
    TechArticle(TechArticle),
    TextDigitalDocument(TextDigitalDocument),
    Thesis(Thesis),
    UserReview(UserReview),
    VideoGallery(VideoGallery),
    VideoGame(VideoGame),
    VideoGameClip(VideoGameClip),
    VideoGameSeries(VideoGameSeries),
    VideoObject(VideoObject),
    VideoObjectSnapshot(VideoObjectSnapshot),
    VisualArtwork(VisualArtwork),
    WPAdBlock(WPAdBlock),
    WPFooter(WPFooter),
    WPHeader(WPHeader),
    WPSideBar(WPSideBar),
    WebApplication(WebApplication),
    WebContent(WebContent),
    WebPage(WebPage),
    WebPageElement(WebPageElement),
    WebSite(WebSite),
}
