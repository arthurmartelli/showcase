use super::{
    args,
    sql::{
        db,
        models::{Song, SongID},
        schema,
    },
};
use diesel::prelude::*;
use rand::prelude::SliceRandom;

pub fn show_ops(argument: args::ShowArguments) {
    match argument.selection {
        Some(selection) => match selection {
            args::ShowOptions::All => show_all_songs(),
            args::ShowOptions::Posted => show_posted_songs(),
            args::ShowOptions::NotPosted => show_not_posted_songs(),
            args::ShowOptions::Recommend => recommend_song(),
            args::ShowOptions::ID(song_id) => show_by_id(song_id),
        },
        None => show_all_songs(),
    }
}

fn show_all_songs() {
    println!("Showing all songs");
    use schema::songs::dsl::*;

    let mut conn: PgConnection = db::establish_connection();

    let result: Vec<Song> = songs
        .order(id.asc())
        .then_order_by(published.asc())
        .load::<Song>(&mut conn)
        .expect("Error loading songs");

    for song in result {
        println!("{}", song.to_string())
    }
}

fn show_posted_songs() {
    println!("Showing posted songs");
    use schema::songs::dsl::*;

    let mut conn: PgConnection = db::establish_connection();

    let result: Vec<Song> = songs
        .filter(published.is_distinct_from(None::<chrono::NaiveDateTime>))
        .load::<Song>(&mut conn)
        .expect("Error loading songs");

    for song in result {
        println!("{}", song.to_string())
    }
}

fn show_not_posted_songs() {
    println!("Showing posted songs");
    use schema::songs::dsl::*;

    let mut conn: PgConnection = db::establish_connection();

    let result: Vec<Song> = songs
        .filter(published.is_not_distinct_from(None::<chrono::NaiveDateTime>))
        .load::<Song>(&mut conn)
        .expect("Error loading songs");

    for song in result {
        println!("{}", song.to_string())
    }
}

fn show_by_id(song_id: SongID) {
    println!("Showing song with ID: {}", song_id.id());
    use schema::songs::dsl::*;

    let mut conn: PgConnection = db::establish_connection();

    let result = songs
        .filter(id.eq(song_id.id()))
        .load::<Song>(&mut conn)
        .expect("Error loading songs");

    let matched_song: Option<&Song> = result.first();

    match matched_song {
        Some(song) => println!("{}", song.to_string()),
        None => println!("No song with this ID"),
    }
}

fn recommend_song() {
    println!("Showing posted songs");
    use schema::songs::dsl::*;

    let mut conn: PgConnection = db::establish_connection();

    let result: Vec<Song> = songs
        .filter(published.is_not_distinct_from(None::<chrono::NaiveDateTime>))
        .load::<Song>(&mut conn)
        .expect("Error loading songs");

    let song: Option<&Song> = result.choose(&mut rand::thread_rng());

    match song {
        Some(song) => println!("{}", song.to_string()),
        None => println!("There is no song to recommend!"),
    }
}
