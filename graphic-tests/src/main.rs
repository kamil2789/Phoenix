use clap::Parser;
use tests::run;

mod args_parser;
mod image;
mod tests;
mod utils;
mod workspace;

fn main() {
    run(&args_parser::Args::parse());
}
