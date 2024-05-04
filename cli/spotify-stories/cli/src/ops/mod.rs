pub mod args;
mod show_ops;
mod song_ops;
pub use crate::{spotify_stories, sql};

pub fn command_handler(command: args::SpotifyCommand) {
    match command {
        args::SpotifyCommand::Song(argument) => song_ops::song_ops(argument),
        args::SpotifyCommand::Show(argument) => show_ops::show_ops(argument),
    }
}
