use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
  let database_url = env::var("DATABASE_URL").unwrap();
  PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
