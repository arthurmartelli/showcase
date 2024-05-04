use clap::{Args, Parser, Subcommand};

use super::sql::models::{NewSong, SongID};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct SpotifyStoriesArgs {
    #[clap(subcommand)]
    pub command: SpotifyCommand,
}

#[derive(Debug, Subcommand)]
pub enum SpotifyCommand {
    /// Edit the saved songs
    Song(SongArguments),

    /// Show all songs
    Show(ShowArguments),
}

#[derive(Debug, Args)]
pub struct SongArguments {
    #[clap(subcommand)]
    pub option: SongOptions,
}

#[derive(Debug, Subcommand)]
pub enum SongOptions {
    /// Create a new song
    Add(NewSong),

    /// Delete a song
    Delete(SongID),

    /// Post a song
    Post(SongID),
}

#[derive(Debug, Args)]
pub struct ShowArguments {
    #[clap(subcommand)]
    pub selection: Option<ShowOptions>,
}

#[derive(Debug, Subcommand)]
pub enum ShowOptions {
    /// Show all songs
    All,

    /// Show posted songs
    Posted,

    /// Show not posted songs
    NotPosted,

    /// Recommend a song to post
    Recommend,

    /// Search a song by id
    ID(SongID),
}
