use super::index;
use crate::auth::{unset_auth_cookie, Auth};
use rocket::{http::Cookies, response::Redirect};

#[get("/sign-out")]
pub fn get(_auth: Auth, cookies: Cookies) -> Redirect {
    unset_auth_cookie(cookies);
    Redirect::to(uri!(index::get))
}
