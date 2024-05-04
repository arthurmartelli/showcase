use super::{
    args, spotify_stories,
    sql::{
        db,
        models::{NewSong, Song, SongID},
        schema,
    },
};

use chrono::{DateTime, Datelike, Local, NaiveDateTime};
use diesel::prelude::*;

pub fn song_ops(argument: args::SongArguments) {
    match argument.option {
        args::SongOptions::Add(song) => add_song(song),
        args::SongOptions::Delete(song) => delete_song(song),
        args::SongOptions::Post(song_id) => post_song(song_id),
    }
}

fn add_song(song: NewSong) {
    println!("Adding song: {}", song.to_string());
    use schema::songs::dsl::*;

    let mut conn: PgConnection = db::establish_connection();

    let existing = songs
        .filter(author.eq(&song.author()))
        .filter(title.eq(&song.title()))
        .load::<Song>(&mut conn)
        .expect("Error loading songs");

    if !existing.is_empty() {
        println!(
            "Cannot add duplicate song (ID: {})",
            existing.first().unwrap().id()
        );
        return;
    }

    diesel::insert_into(songs)
        .values(song)
        .execute(&mut conn)
        .expect("Error saving song");
}

fn delete_song(song_id: SongID) {
    use schema::songs::dsl::*;

    let mut conn: PgConnection = db::establish_connection();

    let song_filtered: Vec<Song> = songs
        .filter(id.eq(song_id.id()))
        .load::<Song>(&mut conn)
        .expect("Error loading songs");

    let song: &Song = song_filtered
        .first()
        .expect(&format!("There is no song with the ID {}", song_id.id()));

    println!("Deleting song: {}", song.to_string());

    if !spotify_stories::ask_for_confirmation() {
        println!("Will not delete the song")
    };

    diesel::delete(songs.find(song_id.id()))
        .execute(&mut conn)
        .expect("Error posting a song");

    println!("Deleted song: {}", song.to_string());
}

fn post_song(song_id: SongID) {
    use schema::songs::dsl::*;

    let current_time: DateTime<Local> = chrono::prelude::Local::now();
    let published_time: NaiveDateTime = chrono::NaiveDateTime::parse_from_str(
        &format!(
            "{}-{}-{} 00:00:00",
            current_time.year(),
            current_time.month(),
            current_time.day()
        ),
        "%Y-%m-%d %H:%M:%S",
    )
    .expect("Unable to parse the date");

    let mut conn: PgConnection = db::establish_connection();

    let song_filtered: Vec<Song> = songs
        .filter(id.eq(song_id.id()))
        .load::<Song>(&mut conn)
        .expect("Error loading songs");

    let song: &Song = song_filtered
        .first()
        .expect(&format!("There is no song with the ID {}", song_id.id()));

    let updated_song: Song = Song::new(
        song.id(),
        song.title().to_string(),
        song.author().to_string(),
        Some(published_time),
    );

    println!("Updating song: {}", song.to_string());

    diesel::update(songs.find(song_id.id()))
        .set(updated_song)
        .execute(&mut conn)
        .expect("Error posting a song");
}
