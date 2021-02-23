use crate::{
    auth::set_auth_cookie,
    db::{account, notepad},
    models::account::AccountData,
    DbConn,
};
use rocket::{http::Cookies, request::Form};
use rocket_contrib::templates::{tera::Context, Template};

#[get("/sign-up")]
pub fn get() -> Template {
    Template::render("sign-up", Context::default())
}

#[post("/sign-up", data = "<account_data>")]
pub fn post(account_data: Form<AccountData>, cookies: Cookies, conn: DbConn) -> Template {
    let created_account = account::create(&*conn, &account_data.username, &account_data.password);

    match created_account {
        Ok(account) => {
            // Initialize account notepad
            notepad::create(&*conn, "your notes...", account.account_id).ok();

            set_auth_cookie(account, cookies);

            Template::render("success", Context::default())
        }
        Err(_) => Template::render("error", Context::default()),
    }
}
