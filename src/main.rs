mod days;
mod utils;

use days::{day_one::day_one, day_two::day_two};

fn main() {
    let run = 2;

    match run {
        1 => day_one::main(),
        2 => day_two::main(),
        _ => (),
    }
}
