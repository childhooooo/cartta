use crate::schema::credentials;
use super::user::User;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
pub struct Credential {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub user_id: i32
}

/*
impl Credential {
    pub fn create (
        user_id: i32,
        email: String,
        password: String,
        conn: &PgConnection
    ) -> Result<Credential, AccountError> {
        diesel::insert_into(credentials::table)
        .values((
            credentials::user_id.eq(user_id),
            credentials::email.eq(&email),
            credentials::password.eq(&password)
        ))
        .get_result::<Credential>(conn)
        .map_err(AccountError::DatabaseError)
    }

    pub fn update (
        id: i32,
        email: String,
        password: String,
        conn: &PgConnection
    ) -> Result<Credential, AccountError> {
        diesel::update(credentials::table.find(id))
        .set((
            credentials::email.eq(&email),
            credentials::password.eq(&password)
        ))
        .get_result::<Credential>(conn)
        .map_err(AccountError::DatabaseError)
    }

    pub fn delete (
        id: i32,
        conn: &PgConnection
    ) -> Result<usize, AccountError> {
        diesel::delete(credentials::table.find(id))
        .execute(conn)
        .map_err(AccountError::DatabaseError)
    }
}
*/