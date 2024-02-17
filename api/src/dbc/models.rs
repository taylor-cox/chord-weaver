use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::dbc::schema::progressions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChordProgression {
  pub id:                  i32,
  pub genre:               String,
  pub artist:              String,
  pub song_name:           String,
  pub num_chords:          i32,
  pub chord_roman_numeral: String,
  pub link:                String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChordProgressionRequestBody {
  pub genre:               String,
  pub artist:              String,
  pub song_name:           String,
  pub num_chords:          i32,
  pub chord_roman_numeral: String,
  pub link:                String,
}
