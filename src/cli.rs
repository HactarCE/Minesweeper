use getopts::Options;

use crate::board::Difficulty;

pub fn get_opts() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help text");
    opts.optflag(
        "1",
        "beginner",
        "play at BEGINNER difficulty (9x9 with 10 mines)",
    );
    opts.optflag(
        "2",
        "intermediate",
        "play at INTERMEDIATE difficulty (16x16 with 40 mines)",
    );
    opts.optflag(
        "3",
        "expert",
        "play at EXPERT difficulty (16x30 with 99 mines)",
    );
    opts.optopt("x", "width", "play with a custom board width", "WIDTH");
    opts.optopt("y", "height", "play with a custom board height", "HEIGHT");
    opts.optopt(
        "m",
        "mines",
        "play with a custom number of mines",
        "MINE_COUNT",
    );
    opts.optopt(
        "d",
        "density",
        "play with a custom mine density",
        "MINE_DENSITY",
    );
    opts
}

pub fn get_difficulty_from_options() -> Result<Difficulty, Option<&'static str>> {
    let env_args: Vec<String> = std::env::args().collect();
    if let Ok(matches) = get_opts().parse(&env_args[1..]) {
        if matches.opt_present("h") {
            return Err(None);
        }
        // Preset
        let mut result: Option<Difficulty> = None;
        {
            if matches.opt_present("1") {
                result = Some(Difficulty::beginner());
            }
            if matches.opt_present("2") {
                if result == None {
                    result = Some(Difficulty::intermediate());
                } else {
                    return Err(Some("Only one difficulty may be specified"));
                }
            }
            if matches.opt_present("3") {
                if result == None {
                    result = Some(Difficulty::expert());
                } else {
                    return Err(Some("Only one difficulty may be specified"));
                }
            }
        }
        // Size
        let mut width: Option<usize> = None;
        let mut height: Option<usize> = None;
        {
            if let Some(width_str) = matches.opt_str("x") {
                match width_str.parse() {
                    Ok(n) => width = Some(n),
                    Err(_) => return Err(Some("Width must be a positive integer")),
                }
            }
            if let Some(height_str) = matches.opt_str("y") {
                match height_str.parse() {
                    Ok(n) => height = Some(n),
                    Err(_) => return Err(Some("Height must be a positive integer")),
                }
            }
        }
        // Mines
        let mut mines: Option<usize> = None;
        let mut density: Option<f32> = None;
        {
            if let Some(mines_str) = matches.opt_str("m") {
                match mines_str.parse() {
                    Ok(n) => mines = Some(n),
                    Err(_) => return Err(Some("Mine count must be a positive integer")),
                }
            }
            if let Some(density_str) = matches.opt_str("d") {
                match density_str.parse() {
                    Ok(n) => density = Some(n),
                    Err(_) => {
                        return Err(Some(
                            "Mine density must be a decimal number greater than 0.0 and no more than 0.5",
                        ))
                    }
                }
            }
        }
        // Handle overiding presets
        match result {
            Some(Difficulty {
                size: (preset_height, preset_width),
                mines: preset_mines,
            }) => {
                if height == None {
                    height = Some(preset_height);
                }
                if width == None {
                    width = Some(preset_width);
                }
                if mines == None {
                    mines = Some(preset_mines);
                }
            }
            None => {
                if let (None, Some(_)) = (height, width) {
                    height = width.clone();
                } else if let (Some(_), None) = (height, width) {
                    width = height.clone();
                }
            }
        }
        // Handle mine count/density conflict
        if let (Some(_), Some(_)) = (mines, density) {
            return Err(Some("Mine count and mine density are mutually exclusive"));
        }
        // Actually make the Board specification
        if let (Some(width), Some(height)) = (width, height) {
            if let Some(density) = density {
                mines = Some(((width * height) as f32 * density).round() as usize);
            }
            if let Some(mines) = mines {
                return Ok(Difficulty {
                    size: (height, width),
                    mines,
                });
            } else {
                return Err(Some(
                    "A number or density of mines is required (use -m or -d)",
                ));
            }
        } else {
            return Err(Some("A board size is required (use -1, -2, -3, -x, or -y)"));
        }
    }
    Err(None)
}

fn err_too_many_difficulties() -> Option<Difficulty> {
    println!("Only one difficulty may be specified");
    println!();
    None
}

fn err_need_positive_integer(argument: &str) -> Option<Difficulty> {
    println!("{} must be a positive integer", argument);
    None
}

pub fn print_usage() {
    let program = std::env::args().next().unwrap();
    let brief = format!("Usage: {} [options]", program);
    println!("{}", get_opts().usage(&brief));
    println!("Board size and mine count/density must be specified. Any of the");
    println!("three preset difficulties specifies both; these can be overridden");
    println!("manually or specified outright using the other arguments.");
    println!();
    println!("The three difficulties (-1, -2, and -3) are mutually exclusive.");
    println!();
    println!("Mine count and mine density are mutually exclusive.");
    println!();
    println!("If width is specified but not height (or vice versa), the board");
    println!("is assumed to be square.");
}
