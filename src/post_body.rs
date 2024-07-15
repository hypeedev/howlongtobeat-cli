use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Struct {
    #[serde(rename = "sortCategory")]
    pub sort_category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RangeYear {
    pub min: String,
    pub max: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Gameplay {
    pub perspective: String,
    pub flow: String,
    pub genre: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RangeTime {
    pub min: Option<u32>,
    pub max: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Games {
    #[serde(rename = "userId")]
    pub user_id: i64,
    pub platform: String,
    #[serde(rename = "sortCategory")]
    pub sort_category: String,
    #[serde(rename = "rangeCategory")]
    pub range_category: String,
    #[serde(rename = "rangeTime")]
    pub range_time: RangeTime,
    pub gameplay: Gameplay,
    #[serde(rename = "rangeYear")]
    pub range_year: RangeYear,
    pub modifier: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SearchOptions {
    pub games: Games,
    pub filter: String,
    pub sort: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Body {
    #[serde(rename = "searchType")]
    pub search_type: String,
    #[serde(rename = "searchTerms")]
    pub search_terms: Vec<String>,
    #[serde(rename = "searchPage")]
    pub search_page: i64,
    pub size: u8,
    #[serde(rename = "searchOptions")]
    pub search_options: SearchOptions,
    #[serde(rename = "useCache")]
    pub use_cache: bool,
}
