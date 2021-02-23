use crate::{auth::Auth, db::notepad, lib::ClientConnection, lib::DbConn};
use rocket_contrib::templates::Template;

#[get("/")]
pub fn get(auth: Auth, conn: DbConn) -> Template {
    let user_note = notepad::read(&*conn, auth.account_id).unwrap().note;

    let context = ClientConnection {
        is_authenticated: true,
        username: Some(auth.username),
        note: Some(user_note),
    };

    Template::render("index", context)
}

#[get("/", rank = 2)]
pub fn get_unauth() -> Template {
    let context = ClientConnection {
        is_authenticated: false,
        username: None,
        note: None,
    };

    Template::render("index", context)
}
