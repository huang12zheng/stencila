use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::movie::Movie;
use super::product::Product;
use super::tv_episode::TVEpisode;
use super::tv_season::TVSeason;
use super::tv_series::TVSeries;

/// The country of origin of something, including products as well as creative  works such as movie and TV content.<br/><br/>  In the case of TV and movie, this would be the country of the principle offices of the production company or individual responsible for the movie. For other kinds of <a class="localLink" href="/CreativeWork">CreativeWork</a> it is difficult to provide fully general guidance, and properties such as <a class="localLink" href="/contentLocation">contentLocation</a> and <a class="localLink" href="/locationCreated">locationCreated</a> may be more applicable.<br/><br/>  In the case of products, the country of origin of the product. The exact interpretation of this may vary by context and product type, and cannot be fully enumerated here.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum countryOfOrigin {
    CreativeWork(CreativeWork),
    Movie(Movie),
    Product(Product),
    TVEpisode(TVEpisode),
    TVSeason(TVSeason),
    TVSeries(TVSeries),
}
