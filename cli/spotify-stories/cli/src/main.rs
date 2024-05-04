extern crate clap;
extern crate dialoguer;
extern crate diesel;

pub mod ops;
pub mod sql;

use clap::Parser;
use ops::args::SpotifyStoriesArgs;
use ops::command_handler;
pub use spotify_stories;

fn main() {
    let args: SpotifyStoriesArgs = SpotifyStoriesArgs::parse();
    command_handler(args.command)
}
