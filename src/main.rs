mod post_result;
mod post_body;
mod duration_formatter;
mod display_time_component;
mod args;

use post_result::PostResult;
use post_body::{Body, SearchOptions, Games, Gameplay};
use duration_formatter::DurationFormatter;
use display_time_component::display_time_component;
use args::{Args, ToggleOption};

use reqwest::Client;
use clap::Parser;
use colored::Colorize;

macro_rules! link {
    ($url:expr, $text:expr) => {
        format!("\x1B]8;;{}\x1B\\{}\x1B]8;;\x1B\\", $url, $text)
    };
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
                range_time: args.range_time.clone(),
                gameplay: Gameplay {
                    perspective: args.perspective.to_string(),
                    flow: args.flow.to_string(),
                    genre: args.genre.to_string(),
                },
                range_year: args.range_year.clone(),
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
            let url = "https://store.steampowered.com/app/".to_owned() + &*game.profile_steam.to_string();
            let label = "[Steam Store Page]".blue().underline();
            formatted_steam_link = format!(" {}", link!(url, label));
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
