use super::{error, index};
use crate::{auth::set_auth_cookie, db::account, models::account::AccountData, DbConn};
use rocket::{http::Cookies, request::Form, response::Redirect};
use rocket_contrib::templates::{tera::Context, Template};

#[get("/sign-in")]
pub fn get() -> Template {
    Template::render("sign-in", Context::default())
}

#[post("/sign-in", data = "<account_data>")]
pub fn post(account_data: Form<AccountData>, cookies: Cookies, conn: DbConn) -> Redirect {
    let signed_account = account::login(&*conn, &account_data.username, &account_data.password);

    match signed_account {
        Ok(account) => {
            set_auth_cookie(account, cookies);

            Redirect::to(uri!(index::get))
        }
        Err(_) => Redirect::to(uri!(error::get)),
    }
}
