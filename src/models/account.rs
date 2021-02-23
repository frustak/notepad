use crate::schema::account;

/// Account model
#[derive(Queryable, Debug)]
pub struct Account {
    pub account_id: i32,
    pub username: String,
    pub password: String,
}

/// Insertable account (to insert in database)
#[derive(Insertable)]
#[table_name = "account"]
pub struct NewAccount<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

/// Account data received from an html form
#[derive(FromForm)]
pub struct AccountData {
    pub username: String,
    pub password: String,
}
