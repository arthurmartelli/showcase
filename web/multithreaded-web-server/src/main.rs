extern crate clap;

use clap::Parser;

mod ops;

fn main() {
    let args = ops::args::ServerOptions::parse();
    ops::command_handler(args);
}
