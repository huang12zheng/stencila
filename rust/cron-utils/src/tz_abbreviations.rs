use once_cell::sync::Lazy;
use std::collections::HashMap;

/// A list of common timezone abbreviations
/// 
/// There is already another crate for this but it was found
/// to be very hard to translate its values into a `cron_tz::Tz`
/// timezone object.
pub static TZ_ABBREVIATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    [
        ("ABST", "Asia/Kuwait"),
        ("ACST", "Australia/Darwin"),
        ("AEST", "Australia/Sydney"),
        ("AFT", "Asia/Baku"),
        ("AK", "America/Anchorage"),
        ("AMT", "Asia/Baku"),
        ("ARBST", "Asia/Muscat"),
        ("ARST", "Asia/Baghdad"),
        ("ART", "America/Buenos Aires"),
        ("AST", "Canada/Atlantic"),
        ("AWST", "Australia/Perth"),
        ("AZOST", "Atlantic/Azores"),
        ("AZT", "Asia/Baku"),
        ("BST", "Asia/Dhaka"),
        ("BTT", "Asia/Dhaka"),
        ("CAST", "America/El_Salvador"),
        ("CAUST", "Australia/Adelaide"),
        ("CBST", "Canada/Atlantic"),
        ("CCST", "Canada/Saskatchewan"),
        ("CEST", "Europe/Sarajevo"),
        ("CET", "Europe/Belgrade"),
        ("CST", "America/Mexico_City"),
        ("CT", "US/Central"),
        ("CVT", "Atlantic/Cape Verde"),
        ("EAST", "Australia/Brisbane"),
        ("EAT", "Asia/Kuwait"),
        ("ECT", "Africa/Brazzaville"),
        ("EEST", "Asia/Jerusalem"),
        ("EET", "Europe/Helsinki"),
        ("EGST", "Africa/Cairo"),
        ("ESAST", "America/Sao_Paulo"),
        ("EST", "US/East-Indiana"),
        ("ET", "US/Eastern"),
        ("FJT", "Pacific/Fiji"),
        ("GET", "Asia/Baku"),
        ("GMT", "Europe/London"),
        ("GNST", "America/Godthab"),
        ("GST", "Africa/Casablanca"),
        ("GTBST", "Europe/Athens"),
        ("HAST", "Pacific/Honolulu"),
        ("IRKT", "Asia/Irkutsk"),
        ("IRST", "Asia/Tehran"),
        ("ISST", "Asia/Jerusalem"),
        ("IST", "Asia/Calcutta"),
        ("JST", "Europe/Athens"),
        ("KRAT", "Asia/Bangkok"),
        ("KST", "Asia/Seoul"),
        ("MEST", "Africa/Cairo"),
        ("MOST", "Africa/Casablanca"),
        ("MSK", "Europe/Moscow"),
        ("MST", "US/Arizona"),
        ("MSTM", "America/Chihuahua"),
        ("MT", "US/Mountain"),
        ("MUT", "Asia/Baku"),
        ("MVST", "America/Godthab"),
        ("MYST", "Asia/Rangoon"),
        ("NCAST", "Asia/Almaty"),
        ("NMST", "Asia/Jerusalem"),
        ("NPT", "Asia/Kathmandu"),
        ("NST", "Canada/Newfoundland"),
        ("NZST", "Pacific/Auckland"),
        ("PETT", "Asia/Kamchatka"),
        ("PHOT", "Pacific/Tongatapu"),
        ("PKT", "Asia/Tashkent"),
        ("PRST", "Canada/Atlantic"),
        ("PSAST", "America/Santiago"),
        ("PST", "US/Pacific"),
        ("PT", "US/Pacific"),
        ("RST", "Europe/Brussels"),
        ("SAPST", "America/Bogota"),
        ("SAST", "Africa/Harare"),
        ("SAWST", "America/Santiago"),
        ("SBT", "Pacific/Guadalcanal"),
        ("SLT", "Asia/Calcutta"),
        ("SMST", "Pacific/Midway"),
        ("SNST", "Asia/Singapore"),
        ("SST", "Africa/Cairo"),
        ("TAST", "Australia/Hobart"),
        ("THA", "Asia/Bangkok"),
        ("TIST", "Asia/Taipei"),
        ("TST", "Asia/Tokyo"),
        ("UST", "Asia/Taipei"),
        ("VLAT", "Asia/Vladivostok"),
        ("VST", "America/Caracas"),
        ("WAST", "Asia/Tashkent"),
        ("WET", "Europe/Amsterdam"),
        ("WPST", "Pacific/Guam"),
        ("YAKT", "Asia/Yakutsk"),
        ("YEKT", "Asia/Tashkent"),
    ]
    .into_iter()
    .collect()
});
