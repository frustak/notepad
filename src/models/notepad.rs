use crate::schema::notepad;
use serde::Serialize;

/// Notepad model
#[derive(Queryable, Identifiable)]
#[primary_key(notepad_id)]
#[table_name = "notepad"]
pub struct Notepad {
    pub notepad_id: i32,
    pub note: String,
    pub account_id: i32,
}

/// Insertable notepad (to insert in database)
#[derive(Insertable)]
#[table_name = "notepad"]
pub struct NewNotepad<'a> {
    pub note: &'a str,
    pub account_id: i32,
}

/// Notepad data received from an html form
#[derive(FromForm, Serialize)]
pub struct NotepadData {
    pub note: String,
}

impl From<Notepad> for NotepadData {
    fn from(notepad: Notepad) -> Self {
        Self { note: notepad.note }
    }
}
