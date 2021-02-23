use crate::{auth::set_auth_cookie, db::account, models::account::AccountData, DbConn};
use rocket::{http::Cookies, request::Form};
use rocket_contrib::templates::{tera::Context, Template};

#[get("/sign-in")]
pub fn get() -> Template {
    Template::render("sign-in", Context::default())
}

#[post("/sign-in", data = "<account_data>")]
pub fn post(account_data: Form<AccountData>, cookies: Cookies, conn: DbConn) -> Template {
    let signed_account = account::login(&*conn, &account_data.username, &account_data.password);

    match signed_account {
        Ok(account) => {
            set_auth_cookie(account, cookies);

            Template::render("success", Context::default())
        }
        Err(why) => {
            error!("{}", why);
            Template::render("error", Context::default())
        }
    }
}
