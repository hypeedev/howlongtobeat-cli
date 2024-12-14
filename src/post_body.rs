use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Gameplay {
    pub perspective: String,
    pub flow: String,
    pub genre: String,
    pub difficulty: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Range {
    pub min: Option<u16>,
    pub max: Option<u16>,
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
    pub range_time: Range,
    pub gameplay: Gameplay,
    #[serde(rename = "rangeYear")]
    pub range_year: Range,
    pub modifier: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Lists {
    #[serde(rename = "sortCategory")]
    pub sort_category: String
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Users {
    #[serde(rename = "sortCategory")]
    pub sort_category: String
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SearchOptions {
    pub games: Games,
    pub filter: String,
    pub sort: u8,
    pub randomizer: u8,
    pub lists: Option<Lists>,
    pub users: Option<Users>
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
