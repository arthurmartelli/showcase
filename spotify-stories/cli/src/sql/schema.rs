// @generated automatically by Diesel CLI.

diesel::table! {
    songs (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        published -> Nullable<Timestamp>,
    }
}
