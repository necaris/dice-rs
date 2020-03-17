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
}

fn main() {
    let opts = Opt::from_args();
    if opts.repl {
        eprintln!("REPL mode isn't implemented yet!");
    }
    match dice::parse_command(&opts.specification.join(" ")) {
        Err(e) => eprintln!("{:?}", e),
        Ok(spec) => println!("{:?}", spec.roll()),
    }
}
