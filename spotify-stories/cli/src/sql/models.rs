use clap::Args;
use diesel::prelude::*;

use super::schema::songs;

#[derive(Debug, Args)]
pub struct SongID {
    id: i32,
}

impl SongID {
    pub fn id(&self) -> i32 {
        self.id
    }
}

#[derive(Debug, Insertable, Args)]
#[diesel(table_name = songs)]
pub struct NewSong {
    title: String,
    author: String,
}

impl NewSong {
    pub fn new(title: String, author: String) -> Self {
        Self { title, author }
    }

    pub fn title(&self) -> &str {
        self.title.as_ref()
    }

    pub fn author(&self) -> &str {
        self.author.as_ref()
    }
}

impl ToString for NewSong {
    fn to_string(&self) -> String {
        format!("{} - {}", self.title, self.author)
    }
}

#[derive(Debug, Queryable, AsChangeset, Args)]
pub struct Song {
    id: i32,
    title: String,
    author: String,
    published: Option<chrono::NaiveDateTime>,
}

impl Song {
    pub fn new(
        id: i32,
        title: String,
        author: String,
        published: Option<chrono::NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            title,
            author,
            published,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        self.title.as_ref()
    }

    pub fn author(&self) -> &str {
        self.author.as_ref()
    }

    pub fn published(&self) -> Option<chrono::NaiveDateTime> {
        self.published
    }
}

impl ToString for Song {
    fn to_string(&self) -> String {
        let publish = match self.published {
            Some(date) => date.format("%Y-%m-%d").to_string(),
            None => String::from("Not Posted"),
        };

        format!(
            "({:03}) [{}] {} - {}",
            self.id, publish, self.title, self.author
        )
    }
}
