use clap::{Parser, ValueEnum};
use clap_num::number_range;
use strum_macros::{Display, EnumString};
use crate::post_body::Range;

#[derive(ValueEnum, Clone, PartialEq, Copy)]
pub(crate) enum ToggleOption {
    Always,
    Never
}

#[derive(ValueEnum, Clone, PartialEq, EnumString, Display)]
pub(crate) enum SortCategory {
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
    #[value(aliases = &["backlog", "backlogs"])]
    MostBacklogs,
    #[strum(serialize = "usersp")]
    #[value(aliases = &["submissions", "beat"])]
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

#[derive(ValueEnum, Clone, EnumString, Display)]
pub(crate) enum Platform {
    #[strum(serialize = "")]
    All,
    #[strum(serialize = "Emulated")]
    Emulated,
    #[strum(serialize = "Nintendo 3DS")]
    #[value(name = "nintendo-3ds", aliases = &["nintendo 3ds", "nintendo3ds", "3ds"])]
    Nintendo3DS,
    #[strum(serialize = "Nintendo Switch")]
    #[value(aliases = &["nintendo switch", "switch"])]
    NintendoSwitch,
    #[value(alias = "desktop")]
    PC,
    #[strum(serialize = "PlayStation 3")]
    #[value(name = "playstation3", aliases = &["playstation 3", "ps3", "ps 3"])]
    PlayStation3,
    #[strum(serialize = "PlayStation 4")]
    #[value(name = "playstation4", aliases = &["playstation 4", "ps4", "ps 4"])]
    PlayStation4,
    #[strum(serialize = "PlayStation 5")]
    #[value(name = "playstation5", aliases = &["playstation 5", "ps5", "ps 5"])]
    PlayStation5,
    #[strum(serialize = "PlayStation Now")]
    #[value(name = "playstation-now", aliases = &["playstation now", "playstationnow", "psnow", "ps now"])]
    PlayStationNow,
    #[strum(serialize = "Wii U")]
    #[value(aliases = &["wii u", "wiiu"])]
    WiiU,
    #[strum(serialize = "Xbox 360")]
    #[value(aliases = &["xbox 360", "xbox-360", "x360", "360"])]
    Xbox360,
    #[strum(serialize = "Xbox One")]
    #[value(aliases = &["xbox one", "xboxone", "xone", "one"])]
    XboxOne,
    #[strum(serialize = "Xbox Series X/S")]
    #[value(aliases = &["xbox xs", "xbox-xs", "xboxxs", "xs"])]
    XboxSeriesXS
}

#[derive(ValueEnum, Clone, EnumString, Display, Debug)]
pub(crate) enum Perspective {
    #[strum(serialize = "")]
    All,
    #[strum(serialize = "First-Person")]
    #[value(aliases = &["firstperson", "first person", "pov"])]
    FirstPerson,
    Isometric,
    Side,
    Text,
    #[strum(serialize = "Third-Person")]
    #[value(aliases = &["thirdperson", "third person"])]
    ThirdPerson,
    #[strum(serialize = "Top-Down")]
    #[value(aliases = &["topdown", "top down"])]
    TopDown,
    #[strum(serialize = "Virtual Reality")]
    #[value(aliases = &["vr", "virtual-reality", "virtualreality"])]
    VirtualReality
}

#[derive(ValueEnum, Clone, PartialEq, EnumString, Display)]
pub(crate) enum Flow {
    #[strum(serialize = "")]
    All,
    Incremental,
    #[strum(serialize = "Massively Multiplayer")]
    #[value(aliases = &["massivelymultiplayer", "massively multiplayer"])]
    MassivelyMultiplayer,
    Multidirectional,
    #[strum(serialize = "On-Rails")]
    #[value(aliases = &["onrails", "on rails"])]
    OnRails,
    #[strum(serialize = "Point-and-Click")]
    #[value(aliases = &["pointandclick", "point and click"])]
    PointAndClick,
    #[strum(serialize = "Real-Time")]
    #[value(aliases = &["realtime", "real time", "rt"])]
    RealTime,
    Scrolling,
    #[strum(serialize = "Turn-Based")]
    #[value(aliases = &["turnbased", "turn based", "turn"])]
    TurnBased
}

#[derive(ValueEnum, Clone, EnumString, Display)]
pub(crate) enum Genre {
    #[strum(serialize = "")]
    All,
    Action,
    Adventure,
    Arcade,
    #[strum(serialize = "Battle Arena")]
    #[value(aliases = &["battle-arena", "battle arena"])]
    BattleArena,
    #[strum(serialize = "Beat em Up")]
    #[value(aliases = &["beat-em-up", "beat em up"])]
    BeatEmUp,
    #[strum(serialize = "Board Game")]
    #[value(aliases = &["board-game", "board game"])]
    BoardGame,
    Breakout,
    #[strum(serialize = "Card Game")]
    #[value(aliases = &["card-game", "card game"])]
    CardGame,
    #[strum(serialize = "City-Building")]
    #[value(aliases = &["city-building", "city building"])]
    CityBuilding,
    Compilation,
    Educational,
    Fighting,
    Fitness,
    Flight,
    #[strum(serialize = "Full Motion Video (FMV)")]
    #[value(aliases = &["full-motion-video", "full motion video", "fmv"])]
    FullMotionVideo,
    #[strum(serialize = "Hack and Slash")]
    #[value(aliases = &["hack-and-slash", "hack and slash"])]
    HackAndSlash,
    #[strum(serialize = "Hidden Object")]
    #[value(aliases = &["hidden-object", "hidden object"])]
    HiddenObject,
    Horror,
    #[strum(serialize = "Interactive Art")]
    #[value(aliases = &["interactive-art", "interactive art"])]
    InteractiveArt,
    Management,
    #[strum(serialize = "Music/Rhythm")]
    #[value(aliases = &["music/rhythm", "music-rhythm", "music rhythm", "music", "rhythm"])]
    MusicRhythm,
    #[strum(serialize = "Open World")]
    #[value(aliases = &["open-world", "open world"])]
    OpenWorld,
    Party,
    Pinball,
    Platform,
    Puzzle,
    #[strum(serialize = "Racing/Driving")]
    #[value(aliases = &["racing/driving", "racing-driving", "racing driving", "racing", "driving"])]
    RacingDriving,
    Roguelike,
    #[strum(serialize = "Role-Playing")]
    #[value(aliases = &["role-playing", "role playing", "rpg"])]
    RolePlaying,
    Sandbox,
    Shooter,
    Simulation,
    Social,
    Sports,
    Stealth,
    #[strum(serialize = "Strategy/Tactical")]
    #[value(aliases = &["strategy/tactical", "strategy-tactical", "strategy tactical", "strategy", "tactical"])]
    StrategyTactical,
    Survival,
    #[strum(serialize = "Tower Defense")]
    #[value(aliases = &["tower-defense", "tower defense", "td"])]
    TowerDefense,
    Trivia,
    #[strum(serialize = "Vehicular Combat")]
    #[value(aliases = &["vehicular-combat", "vehicular combat"])]
    VehicularCombat,
    #[strum(serialize = "Visual Novel")]
    #[value(aliases = &["visual-novel", "visual novel"])]
    VisualNovel
}

fn parse_range(s: &str, min: u16, max: u16) -> Result<Range, String> {
    if s == "" { return Ok(Range { min: None, max: None }) }

    let parts = s.split('-').collect::<Vec<&str>>();
    let mut split: Vec<u16> = Vec::new();

    for part in parts {
        match number_range(part, min, max) {
            Ok(n) => split.push(n),
            Err(e) => return Err(e)
        }
    }

    if split.len() == 1 {
        // set max to 0 if only one value is provided
        split.push(0);
    }

    let min = split[0];
    let max = split[1];

    Ok(Range {
        min: Some(min),
        max: if max == 0 { None } else { Some(max) }
    })
}

fn parse_range_time(s: &str) -> Result<Range, String> {
    parse_range(s, u16::MIN, u16::MAX)
}

fn parse_range_year(s: &str) -> Result<Range, String> {
    parse_range(s, 1958, 2024)
}

#[derive(Parser)]
pub(crate) struct Args {
    pub(crate) search: Vec<String>,
    #[clap(short, long, default_value_t = 5, help = "Number of results to display")]
    pub(crate) size: u8,
    #[clap(short, long, default_value_t = ToggleOption::Always, value_enum, ignore_case = true, help = "Colorize output")]
    pub(crate) color: ToggleOption,
    #[clap(short = 'S', long, default_value_t = SortCategory::MostPopular, value_enum, ignore_case = true, help = "Sort by category")]
    pub(crate) sort: SortCategory,
    #[clap(short, long, default_value_t = false, help = "Reverse sort order")]
    pub(crate) reverse: bool,
    #[clap(long, alias = "year", default_value = "", value_parser=parse_range_year, help = "Range of years to search for, example: 2000-2010")]
    pub(crate) range_year: Range,
    #[clap(short, long, default_value = "all", value_enum, ignore_case = true, help = "Platform to search for")]
    pub(crate) platform: Platform,
    #[clap(short = 'P', long, default_value = "all", value_enum, ignore_case = true, help = "Game perspective")]
    pub(crate) perspective: Perspective,
    #[clap(short, long, default_value = "all", value_enum, ignore_case = true, help = "Game flow")]
    pub(crate) flow: Flow,
    #[clap(short, long, default_value = "all", value_enum, ignore_case = true, help = "Game genre")]
    pub(crate) genre: Genre,
    #[clap(long, alias = "time", default_value = "", value_parser=parse_range_time, help = "Range of time to search for (in hours), example: 10-20")]
    pub(crate) range_time: Range,
    #[clap(long, conflicts_with = "no_dlc", help = "Show only DLCs")]
    pub(crate) dlc: bool,
    #[clap(long, alias = "nodlc", conflicts_with = "dlc", help = "Hide all DLCs")]
    pub(crate) no_dlc: bool,
    #[clap(short = 'I', long, default_value_t = false, help = "Show images (using \"viuer\" crate, full resolution images are displayed only in some terminals)")]
    pub(crate) images: bool,
    #[clap(long, alias = "raw", help = "Output raw JSON")]
    pub(crate) json: bool,
    #[clap(short, long, help = "Show additional information")]
    pub(crate) info: bool
}
