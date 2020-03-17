extern crate structopt;
use structopt::StructOpt;

extern crate dice;

#[derive(Debug, StructOpt)]
#[structopt(name = "dice", about = "An example of StructOpt usage.")]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Enter REPL mode
    #[structopt(short, long)]
    repl: bool,

    /// Dice specification -- e.g. "1d6" or "2d10". Add as many as you like.
    #[structopt(name = "SPECS", required_unless("repl"), multiple = true)]
    specification: Vec<String>,

    // TODO: handle a `profile` name / path for loading a profile map, to
    // refer comparisons against
}

fn main() {
    let opts = Opt::from_args();
    if opts.repl {
        eprintln!("REPL mode isn't implemented yet!");
        return;
    }
    let spec_string = opts.specification.join(" ");
    match dice::parse_command(&spec_string) {
        Err(e) => eprintln!("{:?} (Evaluated: '{}')", e, spec_string),
        Ok(spec) => {
            let rolls = spec.roll();
            for (die, roll) in spec.dice.iter().zip(rolls.iter()) {
                println!("{} (d{})", roll, die.sides);
            }
            let sum: u32 = rolls.iter().map(|r| *r as u32).sum();
            println!(" -- sum: {}", sum);
        }
    }
}
