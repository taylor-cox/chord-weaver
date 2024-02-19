use std::env;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::path::PathBuf;

use axum::http::Method;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{self, TraceLayer};
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tracing::Level;
use tracing_appender::rolling; // Import the necessary module

mod api;
mod dbc;

#[tokio::main]
async fn main() {
  // Init .env if exists.
  dotenv().ok();
  dotenv::from_filename(".spotify-env").ok();

  // Add all the chord routes to the API.
  let chord_routes = Router::new()
    .route(
      "/chord-progression",
      get(api::v0::chord_progression::get_chord_progression),
    )
    .route(
      "/chord-progression",
      post(api::v0::chord_progression::post_chord_progression),
    )
    .route(
      "/chord-progression",
      patch(api::v0::chord_progression::update_chord_progression),
    )
    .route(
      "/chord-progression",
      delete(api::v0::chord_progression::delete_chord_progression),
    );

  // Add all the spotify routes to the API.
  let spotify_routes: Router =
    Router::new().route("/spotify", get(api::v0::spotify::search_song));

  // Implement logging.
  let info_file = rolling::daily("./logs", "info");
  tracing_subscriber::fmt()
    .with_target(false)
    .with_writer(info_file)
    .with_ansi(false)
    .with_max_level(Level::INFO)
    .pretty()
    .init();

  // HTTPS configuration.
  let config = RustlsConfig::from_pem_file(
    PathBuf::from("./ssl-certs").join("cert.pem"),
    PathBuf::from("./ssl-certs").join("key.pem"),
  )
  .await
  .unwrap();

  // Create the app with the API routes.
  let app = Router::new()
    .nest("/", chord_routes)
    .nest("/", spotify_routes)
    .layer(
      TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
        .on_body_chunk(trace::DefaultOnBodyChunk::new())
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO)),
    )
    .layer(ValidateRequestHeaderLayer::accept("application/json"))
    .layer(
      CorsLayer::new() // Use the imported module
        .allow_methods([Method::GET, Method::POST, Method::PATCH])
        .allow_origin(Any),
    );

  // Run with tokio.
  let (ip_addr_arr, port) = get_ip_port_from_env().await;
  let addr = SocketAddr::from((ip_addr_arr, port));
  tracing::debug!("listening on {}", addr);
  // HTTPs and HTTP server.
  // TODO: Remove HTTP server.
  axum_server_dual_protocol::bind_dual_protocol(addr, config)
    .serve(app.clone().into_make_service())
    .await
    .unwrap();
}

async fn get_ip_port_from_env() -> ([u8; 4], u16) {
  // Get the IP and port from the environment variables.
  let ip_arr: [u8; 4] =
    (serde_json::from_str(env::var("URL").unwrap_or("[]".to_string()).as_str())
      as Result<Vec<u8>, _>)
      .unwrap()
      .try_into()
      .unwrap();
  let port = env::var("PORT")
    .unwrap_or("ERROR: PORT not set".to_string())
    .parse::<u16>()
    .unwrap();
  (ip_arr, port)
}
