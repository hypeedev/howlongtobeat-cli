use serde::{Deserialize, Serialize};
use std::time::Duration;

fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let seconds = u64::deserialize(deserializer)?;
    Ok(Duration::from_secs(seconds))
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Game {
    pub game_id: i64,
    pub game_name: String,
    pub game_name_date: i64,
    pub game_alias: String,
    pub game_type: String,
    pub game_image: String,
    pub comp_lvl_combine: i64,
    pub comp_lvl_sp: i64,
    pub comp_lvl_co: i64,
    pub comp_lvl_mp: i64,
    pub comp_lvl_spd: i64,
    #[serde(deserialize_with = "deserialize_duration")]
    pub comp_main: Duration,
    #[serde(deserialize_with = "deserialize_duration")]
    pub comp_plus: Duration,
    #[serde(deserialize_with = "deserialize_duration")]
    pub comp_100: Duration,
    pub comp_all: i64,
    pub comp_main_count: u32,
    pub comp_plus_count: u32,
    pub comp_100_count: u32,
    pub comp_all_count: u32,
    pub invested_co: i64,
    pub invested_mp: i64,
    pub invested_co_count: i64,
    pub invested_mp_count: i64,
    pub count_comp: i64,
    pub count_speedrun: i64,
    pub count_backlog: i64,
    pub count_review: i64,
    pub review_score: i64,
    pub count_playing: i64,
    pub count_retired: i64,
    pub profile_dev: String,
    pub profile_popular: i64,
    pub profile_steam: i64,
    pub profile_platform: String,
    pub release_world: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PostResult {
    pub color: String,
    pub title: String,
    pub category: String,
    pub count: u32,
    #[serde(rename = "pageCurrent")]
    pub page_current: i64,
    #[serde(rename = "pageTotal")]
    pub page_total: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    pub data: Vec<Game>,
    #[serde(rename = "userData")]
    pub user_data: Vec<String>,
    #[serde(rename = "displayModifier")]
    pub display_modifier: Option<String>,
}
