// @generated automatically by Diesel CLI.

diesel::table! {
    progressions (id) {
        id -> Serial,
        genre -> Varchar,
        artist -> Varchar,
        song_name -> Varchar,
        num_chords -> Integer,
        chord_roman_numeral -> Varchar,
        link -> Varchar,
    }
}
