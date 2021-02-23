use super::{error, index};
use crate::{auth::Auth, db::notepad, models::notepad::NotepadData, DbConn};
use rocket::{request::Form, response::Redirect};

#[post("/notepad", data = "<new_notepad>")]
pub fn post(auth: Auth, new_notepad: Form<NotepadData>, conn: DbConn) -> Redirect {
    // Update notepad with new note
    let user_notepad = notepad::update(&*conn, auth.account_id, &new_notepad.note);

    match user_notepad {
        Ok(_) => Redirect::to(uri!(index::get)),
        Err(_) => Redirect::to(uri!(error::get)),
    }
}
