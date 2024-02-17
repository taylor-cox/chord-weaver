use std::env;

use axum::extract::Query;
use axum::Json;
use base64::prelude::*;
use redis::Commands;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct SpotifyQueryParameters {
  #[serde(default)]
  search: String,
}

const REDIS_SPOTIFY_ACCESS_TOKEN: &str = "spotify_token";
const SPOTIFY_ACCESS_TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

pub async fn search_song(
  spotify_query_params: Query<SpotifyQueryParameters>,
) -> Json<Value> {
  // First, check if the cache has the API token.
  tracing::info!("Searching for song: {}", spotify_query_params.search.clone());
  tracing::info!("Opening Redis connection");
  let redis_url = env::var("REDIS_URL").unwrap();
  let client = redis::Client::open(redis_url).unwrap();
  let mut con = client.get_connection().unwrap();

  tracing::info!("Getting spotify token from cache");
  let mut token: String =
    con.get(REDIS_SPOTIFY_ACCESS_TOKEN).unwrap_or("".to_string());

  if token == "" {
    // If the token is not in the cache, get a new token from the Spotify API.
    tracing::info!("Getting new Spotify access token from API.");
    let spotify_access_token = get_spotify_access_token().await;
    token = spotify_access_token.access_token.clone();

    // Add the token to the cache.
    tracing::info!("Adding Spotify access token to cache.");
    let _: () = con
      .set(
        REDIS_SPOTIFY_ACCESS_TOKEN,
        spotify_access_token.access_token.clone(),
      )
      .unwrap();

    // Set the expiration time for the token.
    tracing::info!("Setting expiration time for Spotify access token.");
    let _: () = redis::cmd("EXPIRE")
      .arg(REDIS_SPOTIFY_ACCESS_TOKEN)
      .arg(spotify_access_token.expires_in)
      .query(&mut con)
      .unwrap();
  }

  // Send the search to the Spotify API.
  tracing::info!("Searching Spotify API for song.");
  let uri =
    search_spotify_api(token, spotify_query_params.search.clone()).await;

  return Json::<Value>(serde_json::json!({
    "uri": uri.replace("\"", "")
  }));
}

async fn search_spotify_api(token: String, search: String) -> String {
  let client = reqwest::Client::new();

  // Query the Spotify API.
  let resp: Value = client
    .get(format!(
      "https://api.spotify.com/v1/search?q={}&type=track&limit=1",
      search
    ))
    .header("Authorization", format!("Bearer {}", token))
    .send()
    .await
    .unwrap()
    .json()
    .await
    .unwrap();

  // Return the URI of the track found.
  resp
    .get("tracks")
    .unwrap()
    .get("items")
    .unwrap()
    .get(0)
    .unwrap()
    .get("uri")
    .unwrap()
    .to_string()
}

#[derive(Deserialize, Debug)]
struct SpotifyAccessToken {
  access_token: String,
  // token_type:   String,
  expires_in:   u32,
}

async fn get_spotify_access_token() -> SpotifyAccessToken {
  let client = reqwest::Client::new();
  let auth_token = BASE64_STANDARD.encode(format!(
    "{}:{}",
    env::var("SPOTIFY_CLIENT_ID").unwrap(),
    env::var("SPOTIFY_CLIENT_SECRET").unwrap()
  ));
  let resp = client
    .post(SPOTIFY_ACCESS_TOKEN_URL) // TODO: Move to env var
    .header("Authorization", format!("Basic {}", auth_token))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body("grant_type=client_credentials")
    .send()
    .await
    .unwrap();
  resp.json::<SpotifyAccessToken>().await.unwrap()
}
