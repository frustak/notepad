use rocket_contrib::{database, databases::diesel};

/// Postgres database connection for rocket
#[database("postgres_db")]
pub struct DbConn(pub diesel::PgConnection);
