use colored::Colorize;
use crate::Args;
use crate::duration_formatter::DurationFormatter;
use crate::post_result::Game;

fn get_accuracy_color(count: u32) -> (u8, u8, u8) {
    match count {
        0..=4 => (255, 58, 58),
        5..=9 => (204, 59, 81),
        10..=14 => (130, 73, 133),
        15..=19 => (86, 80, 161),
        20..=24 => (72, 92, 171),
        25..=29 => (58, 109, 181),
        30.. => (40, 127, 194)
    }
}

fn format_with_color(value: &str, count: u32) -> String {
    let color = get_accuracy_color(count);
    value.to_string().truecolor(color.0, color.1, color.2).to_string()
}

pub fn display_time_components(indentation: u8, game: Game, args: &Args) -> u32 {
    let mut lines_printed = 0u32;

    let mut components: Vec<(&str, String, u32)>;
    if game.comp_lvl_combine == 1 {
        components = vec![
            ("Solo: ", game.comp_all.format(), game.comp_all_count),
            ("Co-op:", game.invested_co.format(), game.invested_co_count),
            ("Vs.  :", game.invested_mp.format(), game.invested_mp_count)
        ]
    } else {
        components = vec![
            ("Main Story:   ", game.comp_main.format(), game.comp_main_count),
            ("Main + Extra: ", game.comp_plus.format(), game.comp_plus_count),
            ("Completionist:", game.comp_100.format(), game.comp_100_count)
        ];
    }
    components = components.into_iter().filter(|(_, _, count)| *count > 0).collect();

    let max_length = components.iter().map(|(_, time, _)| time.len())
        .max().unwrap_or(0);

    for (name, time, count) in components {
        let current_length = time.len();
        let required_spaces = max_length - current_length;

        if indentation > 0 { print!("\x1B[{}C", indentation) }

        let mut time = format_with_color(time.as_str(), count);
        if args.info {
            let polled = format!("({})", count).truecolor(120, 120, 120);
            time = format!("{}{} {}", time, " ".repeat(required_spaces), polled);
        }
        println!("{} {}", name.truecolor(200, 200, 200), time);

        lines_printed += 1;
    }

    lines_printed
}
