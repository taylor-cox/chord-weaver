use std::fmt;
use std::str::FromStr;

use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use diesel::prelude::*;
use diesel::{delete, insert_into, SelectableHelper};
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

use crate::dbc::connection::establish_connection;
use crate::dbc::models::{ChordProgression, ChordProgressionRequestBody};
use crate::dbc::schema::progressions::dsl::*;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ChordProgressionQueryParameters {
  #[serde(default, deserialize_with = "empty_string_as_none")]
  id:          Option<i32>,
  song:        Option<String>,
  artist:      Option<String>,
  num_chords:  Option<i32>,
  progression: Option<String>,
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
  D: Deserializer<'de>,
  T: FromStr,
  T::Err: fmt::Display, {
  let opt = Option::<String>::deserialize(de)?;
  match opt.as_deref() {
    None | Some("") => Ok(None),
    Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
  }
}

pub async fn get_chord_progression(
  chord_progression_query_params: Query<ChordProgressionQueryParameters>,
) -> Json<Value> {
  let query_params = chord_progression_query_params.0;

  // Get pagination values from query parameters.
  let pagination_id = query_params.id.unwrap_or(-1);
  let mut pagination_song = query_params.song.unwrap_or("".to_string());
  let mut pagination_artist = query_params.artist.unwrap_or("".to_string());
  let pagination_num_chords = query_params.num_chords.unwrap_or(-1);
  let mut pagination_progression =
    query_params.progression.unwrap_or("".to_string());

  pagination_song = format!("%{}%", pagination_song);
  pagination_artist = format!("%{}%", pagination_artist);
  pagination_progression = format!("%{}%", pagination_progression);

  tracing::info!(
    "GET query parameters:\n\t\t\tid: {:?}\n\t\t\tsong: {:?}\n\t\t\tartist: \
     {:?}\n\t\t\tnum_chords: {:?},\n\t\t\tprogression: {:?}",
    pagination_id,
    pagination_song,
    pagination_artist,
    pagination_num_chords,
    pagination_progression
  );
  query_database_for_chord_progression(
    pagination_id,
    pagination_song,
    pagination_artist,
    pagination_num_chords,
    pagination_progression,
  )
  .await
}

async fn query_database_for_chord_progression(
  pagination_id: i32,
  pagination_song: String,
  pagination_artist: String,
  pagination_num_chords: i32,
  pagination_progression: String,
) -> Json<Value> {
  let connection = &mut establish_connection();
  // Query if there is no id and num_chords.
  if pagination_id == -1 && pagination_num_chords == -1 {
    return Json(
      serde_json::to_value(
        &progressions
          .filter(song_name.like(pagination_song))
          .filter(artist.like(pagination_artist))
          .filter(chord_roman_numeral.like(pagination_progression))
          .select(ChordProgression::as_select())
          .load(connection)
          .expect("Error loading chord progressions"),
      )
      .unwrap(),
    );
  }

  // Query if there is no id.
  if pagination_id == -1 {
    return Json(
      serde_json::to_value(
        &progressions
          .filter(song_name.like(pagination_song))
          .filter(artist.like(pagination_artist))
          .filter(num_chords.eq(pagination_num_chords))
          .filter(chord_roman_numeral.like(pagination_progression))
          .select(ChordProgression::as_select())
          .load(connection)
          .expect("Error loading chord progressions"),
      )
      .unwrap(),
    );
  }

  // Query if there is no num_chords.
  if pagination_num_chords == -1 {
    return Json(
      serde_json::to_value(
        &progressions
          .filter(id.eq(pagination_id))
          .filter(song_name.like(pagination_song))
          .filter(artist.like(pagination_artist))
          .filter(chord_roman_numeral.like(pagination_progression))
          .select(ChordProgression::as_select())
          .load(connection)
          .expect("Error loading chord progressions"),
      )
      .unwrap(),
    );
  }

  // Query if there is num_chords and id.
  Json(
    serde_json::to_value(
      &progressions
        .filter(id.eq(pagination_id))
        .filter(song_name.like(pagination_song))
        .filter(artist.like(pagination_artist))
        .filter(num_chords.ne(pagination_num_chords))
        .filter(chord_roman_numeral.like(pagination_progression))
        .select(ChordProgression::as_select())
        .load(connection)
        .expect("Error loading chord progressions"),
    )
    .unwrap(),
  )
}

pub async fn post_chord_progression(
  Json(payload): Json<ChordProgressionRequestBody>,
) -> impl IntoResponse {
  let connection = &mut establish_connection();
  tracing::info!("POST query parameters: {:?}", payload);

  let res = insert_into(progressions)
    .values((
      genre.eq(payload.genre),
      artist.eq(payload.artist.clone()),
      song_name.eq(payload.song_name.clone()),
      num_chords.eq(payload.num_chords),
      chord_roman_numeral.eq(payload.chord_roman_numeral.clone()),
      link.eq(payload.link),
    ))
    .execute(connection)
    .expect("Error inserting chord progression");

  let created_id = query_database_for_chord_progression(
    -1,
    payload.song_name,
    payload.artist,
    payload.num_chords,
    payload.chord_roman_numeral,
  )
  .await
  .0
  .as_array()
  .unwrap()
  .last()
  .unwrap()
  .get("id")
  .unwrap()
  .as_i64()
  .unwrap() as i32;

  tracing::info!("Inserted # chord progression {}", res);
  (
    StatusCode::CREATED,
    format!("Chord progression created, id = {:?}.", created_id),
  )
}

pub async fn delete_chord_progression(
  Json(payload): Json<ChordProgressionRequestBody>,
) -> impl IntoResponse {
  tracing::info!("DELETE payload: {:?}", payload);
  let connection = &mut establish_connection();

  let res = delete(progressions)
    .filter(genre.eq(payload.genre))
    .filter(artist.eq(payload.artist.clone()))
    .filter(song_name.eq(payload.song_name.clone()))
    .filter(num_chords.eq(payload.num_chords))
    .filter(chord_roman_numeral.eq(payload.chord_roman_numeral.clone()))
    .filter(link.eq(payload.link))
    .execute(connection)
    .expect("Error deleting chord progression");

  let deleted_id = query_database_for_chord_progression(
    -1,
    payload.song_name,
    payload.artist,
    payload.num_chords,
    payload.chord_roman_numeral,
  )
  .await
  .0
  .as_array()
  .unwrap()
  .last()
  .unwrap()
  .get("id")
  .unwrap()
  .as_i64()
  .unwrap() as i32;

  tracing::info!("Deleted # chord progression {}", res);

  (StatusCode::OK, format!("Chord progression deleted, id = {}.", deleted_id))
}

pub async fn update_chord_progression(
  Json(payload): Json<ChordProgressionRequestBody>,
) -> Json<Value> {
  // let connection = &mut establish_connection();
  println!("PATCH payload: {:?}", payload);
  tracing::info!("PATCH payload: {:?}", payload);

  todo!("Implement update_chord_progression");
  // let result = progressions
  //   .filter(id.eq(pagination_id))
  //   .limit(1)
  //   .select(ChordProgression::as_select())
  //   .load(connection)
  //   .expect("Error loading chord progressions")
  //   .pop();
  // Json(json!({ "hello": "world" }))
}
