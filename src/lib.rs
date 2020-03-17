extern crate rand;
extern crate nom;

mod die;
mod parser;

pub use crate::die::*;
pub use crate::parser::parse_command;
