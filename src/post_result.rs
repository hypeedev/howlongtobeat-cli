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
    pub comp_lvl_sp: u8,
    pub comp_lvl_co: u8,
    pub comp_lvl_mp: u8,
    pub comp_lvl_spd: u8,
    #[serde(deserialize_with = "deserialize_duration")]
    pub comp_main: Duration,
    #[serde(deserialize_with = "deserialize_duration")]
    pub comp_plus: Duration,
    #[serde(deserialize_with = "deserialize_duration")]
    pub comp_100: Duration,
    #[serde(deserialize_with = "deserialize_duration")]
    pub comp_all: Duration,
    pub comp_main_count: u32,
    pub comp_plus_count: u32,
    pub comp_100_count: u32,
    pub comp_all_count: u32,
    #[serde(deserialize_with = "deserialize_duration")]
    pub invested_co: Duration,
    #[serde(deserialize_with = "deserialize_duration")]
    pub invested_mp: Duration,
    pub invested_co_count: u32,
    pub invested_mp_count: u32,
    pub count_comp: u32,
    pub count_speedrun: u32,
    pub count_backlog: u32,
    pub count_review: u32,
    pub review_score: u8,
    pub count_playing: u32,
    pub count_retired: u32,
    pub profile_dev: String,
    pub profile_popular: u32,
    pub profile_steam: u32,
    pub profile_platform: String,
    pub release_world: u16,
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
