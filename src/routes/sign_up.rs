use super::{error, index};
use crate::{
    auth::set_auth_cookie,
    db::{account, notepad},
    models::account::AccountData,
    DbConn,
};
use rocket::{http::Cookies, request::Form, response::Redirect};
use rocket_contrib::templates::{tera::Context, Template};

#[get("/sign-up")]
pub fn get() -> Template {
    Template::render("sign-up", Context::default())
}

#[post("/sign-up", data = "<account_data>")]
pub fn post(account_data: Form<AccountData>, cookies: Cookies, conn: DbConn) -> Redirect {
    let created_account = account::create(&*conn, &account_data.username, &account_data.password);

    match created_account {
        Ok(account) => {
            // Initialize account notepad
            notepad::create(&*conn, "your notes...", account.account_id).ok();

            set_auth_cookie(account, cookies);

            Redirect::to(uri!(index::get))
        }
        Err(_) => Redirect::to(uri!(error::get)),
    }
}
