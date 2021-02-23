use super::index;
use crate::{auth::Auth, db::notepad, models::notepad::NotepadData, DbConn};
use rocket::{request::Form, response::Redirect};
use rocket_contrib::templates::{tera::Context, Template};

#[get("/notepad")]
pub fn get(auth: Auth, conn: DbConn) -> Template {
    // Get current account notepad
    let user_notepad = notepad::read(&*conn, auth.account_id);

    match user_notepad {
        Ok(user_notepad) => {
            let notepad_data: NotepadData = user_notepad.into();

            Template::render("notepad", notepad_data)
        }
        Err(_) => Template::render("error", Context::default()),
    }
}

#[get("/notepad", rank = 2)]
pub fn get_unauth() -> Redirect {
    Redirect::to(uri!(index::get))
}

#[post("/notepad", data = "<new_notepad>")]
pub fn post(auth: Auth, new_notepad: Form<NotepadData>, conn: DbConn) -> Template {
    // Update notepad with new note
    let user_notepad = notepad::update(&*conn, auth.account_id, &new_notepad.note);

    match user_notepad {
        Ok(user_notepad) => {
            let notepad_data: NotepadData = user_notepad.into();

            Template::render("notepad", notepad_data)
        }
        Err(_) => Template::render("error", Context::default()),
    }
}
