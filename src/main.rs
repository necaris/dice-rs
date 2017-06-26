extern crate clap;
extern crate rand;

use clap::{Arg, App, SubCommand};
use rand::distributions::{IndependentSample, Range};

// fn main() {
//     let between = Range::new(10, 10000);
//     let mut rng = rand::thread_rng();
//     let mut sum = 0;
//     for _ in 0..1000 {
//         sum += between.ind_sample(&mut rng);
//     }
//     println!("{}", sum);
// }

fn is_digit(v: String) -> Result<(), String> {
    match v.parse::<i32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Could not parse argument as integer!"))
    }
}

fn as_int(v: &str) -> Result<i32, std::num::ParseIntError> {
    v.parse::<i32>()
}

fn main() {
    let matches = App::new("Generic Dice Roller")
        .version("0.1")
        .author("Rami Chowdhury <rami.chowdhury@gmail.com>")
        .about("Rolls some fake dice")
        .arg(Arg::with_name("sides")
             .short("s")
             .long("sides")
             .help("Chooses the number of sides on each die")
             .validator(is_digit)
             .default_value("6"))
        .arg(Arg::with_name("dice")
             .short("d")
             .long("dice")
             .help("Choose the number of dice to roll")
             .validator(is_digit)
             .default_value("1"))
        .get_matches();

    let dice = matches.value_of("dice").map(as_int).ok();
    let sides = matches.value_of("sides").map(as_int).ok();

    println!("Rolling {} dice with {} sides",
             dice, sides);
}
