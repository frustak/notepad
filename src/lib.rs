use rocket_contrib::{database, databases::diesel};
use serde::Serialize;

/// Postgres database connection for rocket
#[database("postgres_db")]
pub struct DbConn(pub diesel::PgConnection);

/// A client connection to server, could be either authenticated or not
#[derive(Serialize)]
pub struct ClientConnection {
    pub is_authenticated: bool,
    pub username: Option<String>,
    pub note: Option<String>
}
