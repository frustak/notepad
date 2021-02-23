use crate::{
    models::account::{Account, NewAccount},
    schema::account,
};
use diesel::prelude::*;

/// Create (insert) a new account to account table
pub fn create(
    conn: &PgConnection,
    username: &str,
    password: &str,
) -> Result<Account, &'static str> {
    let password =
        bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|_| "Error hashing password")?;

    let new_account = NewAccount {
        username,
        password: &password,
    };

    diesel::insert_into(account::table)
        .values(new_account)
        .get_result(conn)
        .map_err(|_| "Error creating account")
}

/// Login account with given information
pub fn login(conn: &PgConnection, username: &str, password: &str) -> Result<Account, &'static str> {
    let account = account::table
        .filter(account::username.eq(username))
        .get_result::<Account>(conn)
        .map_err(|_| "No match for account")?;

    let verified =
        bcrypt::verify(password, &account.password).map_err(|_| "Error verifying password")?;

    if verified {
        Ok(account)
    } else {
        Err("Password does not match")
    }
}
