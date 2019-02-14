pub mod user;
pub mod credential;
pub mod error;

use bcrypt::*;
use diesel::PgConnection;
use diesel::{self, prelude::*};
use validator::Validate;

use crate::schema::{users, credentials};

use self::user::User;
use self::error::AccountError;

#[derive(Deserialize, Validate)]
pub struct Account {
    #[validate(length(min = "1", max = "20"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "8", max = "100"))]
    pub password: String
}

pub fn create_user(
    account: Account,
    conn: &PgConnection
) -> Result<User, AccountError> {
    account.validate()?;
    let user =
        diesel::insert_into(users::table)
        .values(users::name.eq(&account.name))
        .get_result::<User>(conn)
        .map_err(AccountError::DatabaseError)?;

    let password_digest = hash(&account.password, DEFAULT_COST)?;
    diesel::insert_into(credentials::table)
    .values((
        credentials::email.eq(&account.email),
        credentials::password.eq(&password_digest),
        credentials::user_id.eq(&user.id)
    ))
    .execute(conn)
    .map_err(AccountError::DatabaseError)?;

    Ok(user)
}

pub fn get_user(
    user_id: i32,
    conn: &PgConnection
) -> Result<User, AccountError> {
    let user =
        users::table
        .find(user_id)
        .first::<User>(conn)
        .optional()
        .map_err(AccountError::DatabaseError)?;

    user.ok_or(AccountError::NotFound)
}

pub fn list_users (
    conn: &PgConnection
) -> Result<Vec<User>, AccountError> {
    users::table
    .order(users::name.asc())
    .load::<User>(conn)
    .map_err(AccountError::DatabaseError)
}

pub fn update_user (
    user_id: i32,
    name: String,
    conn: &PgConnection
) -> Result<User, AccountError> {
    diesel::update(users::table.find(user_id))
    .set((
        users::name.eq(&name)
    ))
    .get_result::<User>(conn)
    .map_err(AccountError::DatabaseError)
}

pub fn delete_user (
    user_id: i32,
    conn: &PgConnection
) -> Result<usize, AccountError> {
    diesel:: delete(users::table.find(user_id))
    .execute(conn)
    .map_err(AccountError::DatabaseError)
}

pub fn check_if_unique (
    name: &str,
    conn: &PgConnection
) -> Result<bool, AccountError> {
    let result =
        users::table
        .filter(users::name.eq(name))
        .execute(conn)
        .map_err(AccountError::DatabaseError)?;

    match result {
        0 => Ok(true),
        _ => Ok(false)
    }
}

pub fn authenticate (
    account: &Account,
    conn: &PgConnection
) -> Result<User, AccountError> {
    let user_pass =
        users::table
        .inner_join(credentials::table)
        .filter(users::name.eq(&account.name))
        .or_filter(credentials::email.eq(&account.email))
        .select((
            (users::id, users::name),
            credentials::password
        ))
        .first::<(User, String)>(conn)
        .optional()
        .map_err(AccountError::DatabaseError)?;

    if let Some((u, p)) = user_pass {
        if verify(&account.password, &p)? {
            Ok(u)
        } else {
            Err(AccountError::IncorrectPassword)
        }
    } else {
        Err(AccountError::NotFound)
    }
}