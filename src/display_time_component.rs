use colored::Colorize;
use crate::Args;

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

pub fn display_time_component(indentation: u8, comp_name: &str, comp_count: u32, comp_format: &str, args: &Args) -> u32 {
    if comp_count > 0 {
        if indentation > 0 { print!("\x1B[{}C", indentation) }
        print!("{} {}", comp_name.truecolor(200, 200, 200), format_with_color(&comp_format, comp_count));
        let mut polled = "".to_string();
        if args.info {
            polled = format!("\t({})", comp_count).truecolor(120, 120, 120).to_string();
        }
        println!("{}", polled);
        1
    } else {
        0
    }
}
