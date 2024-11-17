mod days;
mod utils;

use days::day_one::day_one;

fn main() {
    let run = 1;

    match run {
        1 => day_one::main(),
        _ => (),
    }
}
