#![crate_name = "dice"]
// Specify the type of output artifact.
#![crate_type = "bin"]

extern crate clap;
extern crate rand;

use clap::{Arg, App};
mod die;
use self::die::Die;

fn test_is_positive_integer(v: String) -> Result<(), String> {
    match v.parse::<u32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from(format!("{} is not an integer!", v)))
    }
}

fn as_uint(v: &str) -> u32 {
    v.parse::<u32>().unwrap()
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
             .validator(test_is_positive_integer)
             .default_value("6"))
        .arg(Arg::with_name("dice")
             .short("d")
             .long("dice")
             .help("Choose the number of dice to roll")
             .validator(test_is_positive_integer)
             .default_value("1"))
        .get_matches();

    // Safe to `.unwrap()` here because arguments have been validated
    let dice = matches.value_of("dice").map(as_uint).unwrap();
    let sides = matches.value_of("sides").map(as_uint).unwrap();

    let mut die = Die::new(sides);
    println!("{} {}", dice, die);


    // Print spaces after all but the final roll
    print!("    ");
    for _ in 0..(dice - 1) {
        print!("{}   ", die.roll());
    }
    println!("{}", die.roll());
}
