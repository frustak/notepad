use crate::{
    models::notepad::{NewNotepad, Notepad},
    schema::notepad,
};
use diesel::prelude::*;

/// Create (insert) a new notepad to notepad table with given account id
pub fn create(conn: &PgConnection, note: &str, account_id: i32) -> Result<Notepad, &'static str> {
    let new_notepad = NewNotepad { note, account_id };

    diesel::insert_into(notepad::table)
        .values(new_notepad)
        .get_result(conn)
        .map_err(|_| "Error creating new notepad")
}

/// Find an account notepad based on account id
pub fn read(conn: &PgConnection, account_id: i32) -> Result<Notepad, &'static str> {
    notepad::table
        .filter(notepad::account_id.eq(account_id))
        .get_result(conn)
        .map_err(|_| "No note found for user")
}

/// Update account notepad with new note
pub fn update(
    conn: &PgConnection,
    account_id: i32,
    new_note: &str,
) -> Result<Notepad, &'static str> {
    let target_notepad = read(conn, account_id).unwrap();

    diesel::update(&target_notepad)
        .set(notepad::note.eq(new_note))
        .get_result(conn)
        .map_err(|_| "Error updating note")
}
