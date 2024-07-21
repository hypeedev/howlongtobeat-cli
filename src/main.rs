mod post_result;
mod post_body;
mod duration_formatter;
mod display_time_components;
mod args;

use post_result::PostResult;
use post_body::{Body, SearchOptions, Games, Gameplay};
use display_time_components::display_time_components;
use args::{Args, ToggleOption};

use reqwest::{Client, ClientBuilder};
use clap::Parser;
use colored::{ColoredString, Colorize};
use viuer::Config;
use image::DynamicImage;
use futures::future::join_all;

macro_rules! link {
    ($url:expr, $text:expr) => {
        format!("\x1B]8;;{}\x1B\\{}\x1B]8;;\x1B\\", $url, $text)
    };
}

fn get_terminal_image_dimensions(image: &DynamicImage) -> (u32, u32) {
    let ratio = image.width() as f32 / image.height() as f32;
    let width: u32;
    let height: u32;
    if ratio > 1.0 {
        height = 7u32;
        width = (height as f32 * ratio) as u32;
    } else {
        width = 10u32;
        height = (width as f32 / ratio) as u32;
    }
    (width, (height as f32 / 2.15).round() as u32)
}

async fn fetch_images(client: Client, urls: Vec<String>) -> Vec<DynamicImage> {
    let futures = urls.into_iter().map(|url| {
        let client = client.clone();
        async move {
            let image = client.get(url).send().await.unwrap().bytes().await.unwrap();
            let image = image::load_from_memory(&image).unwrap();
            let image = DynamicImage::ImageRgba8(image.to_rgba8());
            image
        }
    });
    join_all(futures).await
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

    let client = ClientBuilder::new()
        .default_headers(
            reqwest::header::HeaderMap::from_iter(
                vec![
                    ("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0"),
                    ("Referer", "https://howlongtobeat.com"),
                    ("Content-Type", "application/json"),
                ].into_iter().map(|(k, v)| (k.parse().unwrap(), v.parse().unwrap()))
            )
        )
        .build().unwrap();
    let res = client
        .post("https://howlongtobeat.com/api/search")
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await.unwrap().json::<PostResult>().await.unwrap();

    if args.json {
        println!("{}", serde_json::to_string(&res).unwrap());
        return;
    }

    let mut images: Vec<DynamicImage> = Vec::new();
    if args.images {
        let urls: Vec<String> = res.data.iter().map(|game| format!("https://howlongtobeat.com/games/{}?width=100", game.game_image)).collect();
        images = fetch_images(client, urls.clone()).await;
    }

    println!("Found {} Game{}", res.count, if res.count == 1 { "" } else { "s" });

    for (index, game) in res.data.into_iter().enumerate() {
        println!();

        let mut width: u32 = 0;
        let mut height: u32 = 0;
        if args.images {
            let image = &images[index];
            (width, height) = get_terminal_image_dimensions(image);

            let conf = Config {
                absolute_offset: false,
                width: Some(width),
                height: Some(height),
                ..Default::default()
            };

            viuer::print(image, &conf).unwrap();

            // move cursor to the top of the image
            print!("\x1B[{}A", height);
        }

        let mut lines_printed = 0;

        let print_indentation_if_images = || {
            if !args.images { return }
            // move cursor to the right of the image
            print!("\x1B[{}C", width + 1);
        };

        macro_rules! println {
            () => {
                print_indentation_if_images();
                print!("\n");
                lines_printed += 1;
            };
            ($($arg:tt)*) => {
                print_indentation_if_images();
                print!($($arg)*);
                print!("\n");
                lines_printed += 1;
            };
        }

        let mut formatted_game_name = game.game_name.bold();
        if (&args.search).join(" ").to_lowercase() == formatted_game_name.to_lowercase() {
            formatted_game_name = formatted_game_name.green();
        }

        if game.profile_steam != 0 {
            let url = "https://store.steampowered.com/app/".to_owned() + &*game.profile_steam.to_string();
            let label = "[Steam Store Page]".blue().underline();
            formatted_game_name = ColoredString::from(format!("{} {}", formatted_game_name, link!(url, label)));
        }

        println!("{}", formatted_game_name);

        if args.info {
            println!("{} {}", "Developer:".truecolor(200, 200, 200), game.profile_dev);
        }

        if args.info {
            let all_players = game.count_comp + game.count_backlog + game.count_retired;
            let components = vec![
                ("Playing:", game.count_playing.to_string()),
                ("Backlogs:", game.count_backlog.to_string()),
                ("Retired:", format!("{:0.1}%", game.count_retired as f32 / all_players as f32 * 100.0)),
                ("Rating:", format!("{}%", game.review_score)),
                ("Beat:", game.count_comp.to_string())
            ]
                .into_iter()
                .map(|(label, count)| format!("{} {}", label.truecolor(200, 200, 200), count))
                .collect::<Vec<String>>()
                .join(", ");
            println!("{}", components);
        }

        let indentation = if args.images { (width + 1) as u8 } else { 0u8 };
        lines_printed += display_time_components(indentation, game, &args);

        if args.images && height > lines_printed && height - lines_printed > 0 {
            print!("{}", "\n".repeat((height - lines_printed) as usize));
        }
    }
}
