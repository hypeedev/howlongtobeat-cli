mod post_result;
mod post_body;
mod duration_formatter;
mod display_time_component;

use post_result::PostResult;
use post_body::{Body, SearchOptions, Games, RangeTime, Gameplay, RangeYear};
use duration_formatter::DurationFormatter;
use display_time_component::display_time_component;

use std::cmp::PartialEq;
use reqwest::Client;
use clap::{Parser, ValueEnum};
use clap_num::number_range;
use colored::Colorize;
use strum_macros::{Display, EnumString};

#[derive(ValueEnum, Clone, PartialEq, Copy)]
enum ToggleOption {
    Always,
    Never
}

#[derive(ValueEnum, Clone, PartialEq, EnumString, Display)]
enum SortCategory {
    #[strum(serialize = "name")]
    Name,
    #[strum(serialize = "main")]
    Main,
    #[strum(serialize = "mainp")]
    #[value(aliases = &["extras", "extra"])]
    MainExtras,
    #[strum(serialize = "comp")]
    #[value(aliases = &["comp", "completion"])]
    Completionist,
    #[strum(serialize = "averagea")]
    #[value(alias = "time")]
    AverageTime,
    #[strum(serialize = "rating")]
    #[value(alias = "rating")]
    TopRated,
    #[strum(serialize = "popular")]
    #[value(aliases = &["popular", "popularity"])]
    MostPopular,
    #[strum(serialize = "backlog")]
    #[value(alias = "backlog")]
    MostBacklogs,
    #[strum(serialize = "usersp")]
    #[value(alias = "submissions")]
    MostSubmissions,
    #[strum(serialize = "playing")]
    #[value(aliases = &["playing", "played"])]
    MostPlayed,
    #[strum(serialize = "speedruns")]
    #[value(alias = "speedrun")]
    MostSpeedruns,
    #[strum(serialize = "reviews")]
    #[value(alias = "reviews")]
    MostReviews,
    #[strum(serialize = "release")]
    #[value(alias = "release")]
    ReleaseDate
}

fn parse_year(s: &str) -> Result<u16, String> {
    number_range(s, 1958, 2024)
}

#[derive(ValueEnum, Clone, EnumString, Display)]
enum Platform {
    #[strum(serialize = "")]
    All,
    #[strum(serialize = "Emulated")]
    Emulated,
    #[strum(serialize = "Nintendo 3DS")]
    #[value(name = "nintendo-3ds", aliases = &["nintendo3ds", "3ds"])]
    Nintendo3DS,
    #[strum(serialize = "Nintendo Switch")]
    #[value(alias = "switch")]
    NintendoSwitch,
    PC,
    #[strum(serialize = "PlayStation 3")]
    #[value(name = "playstation3", alias = "ps3")]
    PlayStation3,
    #[strum(serialize = "PlayStation 4")]
    #[value(name = "playstation4", alias = "ps4")]
    PlayStation4,
    #[strum(serialize = "PlayStation 5")]
    #[value(name = "playstation5", alias = "ps5")]
    PlayStation5,
    #[strum(serialize = "PlayStation Now")]
    #[value(name = "playstation-now", aliases = &["playstationnow", "psnow"])]
    PlayStationNow,
    #[strum(serialize = "Wii U")]
    #[value(alias = "wiiu")]
    WiiU,
    #[strum(serialize = "Xbox 360")]
    #[value(aliases = &["xbox-360", "x360", "360"])]
    Xbox360,
    #[strum(serialize = "Xbox One")]
    #[value(aliases = &["xboxone", "xone", "one"])]
    XboxOne,
    #[strum(serialize = "Xbox Series X/S")]
    #[value(aliases = &["xbox-xs", "xboxxs", "xs"])]
    XboxSeriesXS
}

#[derive(Parser)]
struct Args {
    search: Vec<String>,
    #[clap(short, long, default_value_t = 5, help = "Number of results to display")]
    size: u8,
    #[clap(short, long, default_value_t = ToggleOption::Always, value_enum, help = "Colorize output")]
    color: ToggleOption,
    #[clap(short = 'S', long, default_value_t = SortCategory::MostPopular, value_enum, help = "Sort by category")]
    sort: SortCategory,
    #[clap(short, long, default_value_t = false, help = "Reverse sort order")]
    reverse: bool,
    #[clap(long, alias = "min-year", default_value_t = 1958, value_parser=parse_year, help = "Minimum release year")]
    year_min: u16,
    #[clap(long, alias = "max-year", default_value_t = 2024, value_parser=parse_year, help = "Maximum release year")]
    year_max: u16,
    #[clap(short, long, default_value = "all", value_enum, help = "Platform to search for")]
    platform: Platform,
    #[clap(long, conflicts_with = "no_dlc", help = "Show only DLCs")]
    dlc: bool,
    #[clap(long, alias = "nodlc", conflicts_with = "dlc", help = "Hide all DLCs")]
    no_dlc: bool,
    #[clap(long, alias = "raw", help = "Output raw JSON")]
    json: bool,
    #[clap(short, long, help = "Show additional information")]
    info: bool
}

#[tokio::main]
async fn main() {
    let mut args = Args::parse();

    let mut new_search = Vec::new();
    for arg in &args.search {
        if arg.contains(" ") {
            let split = arg.split_whitespace();
            new_search.extend(split.map(|s| s.to_string()));
        } else {
            new_search.push(arg.clone());
        }
    }
    args.search = new_search;

    if args.color == ToggleOption::Never { colored::control::set_override(false) }

    let body = Body {
        search_type: "games".to_string(),
        search_terms: args.search.clone(),
        search_page: 1,
        size: args.size,
        search_options: SearchOptions {
            games: Games {
                user_id: 0,
                platform: args.platform.to_string(),
                sort_category: args.sort.to_string(),
                range_category: "main".to_string(),
                range_time: RangeTime {
                    min: None,
                    max: None,
                },
                gameplay: Gameplay {
                    perspective: "".to_string(),
                    flow: "".to_string(),
                    genre: "".to_string(),
                },
                range_year: RangeYear {
                    min: args.year_min.to_string(),
                    max: args.year_max.to_string(),
                },
                modifier: if args.dlc { "only_dlc" } else if args.no_dlc { "hide_dlc" } else { "Modifiers" }.to_string(),
            },
            filter: "".to_string(),
            sort: args.reverse as u8
        },
        use_cache: true,
    };

    let client = Client::new();
    let res = client
        .post("https://howlongtobeat.com/api/search")
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0")
        .header("Referer", "https://howlongtobeat.com")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await.unwrap().json::<PostResult>().await.unwrap();

    if args.json {
        println!("{}", serde_json::to_string(&res).unwrap());
        return;
    }

    println!("Found {} Game{}", res.count, if res.count == 1 { "" } else { "s" });

    for game in res.data {
        println!();

        let mut formatted_game_name = game.game_name.bold();
        if (&args.search).join(" ").to_lowercase() == formatted_game_name.to_lowercase() {
            formatted_game_name = formatted_game_name.green();
        }
        print!("{}", formatted_game_name);

        let mut formatted_steam_link = "".to_string();
        if game.profile_steam != 0 {
            formatted_steam_link = format!(" \x1B]8;;https://store.steampowered.com/app/{}\x1B\\{}\x1B]8;;\x1B\\", game.profile_steam, "[Steam Store Page]".blue().underline());
        }
        println!("{}", formatted_steam_link);

        if args.info {
            println!("{} {}", "Developer:".truecolor(200, 200, 200), game.profile_dev);
        }

        display_time_component("Main Story", game.comp_main_count, game.comp_main.format().as_str(), &args);
        display_time_component("Main + Extra", game.comp_plus_count, game.comp_plus.format().as_str(), &args);
        display_time_component("Completionist", game.comp_100_count, game.comp_100.format().as_str(), &args);
    }
}
