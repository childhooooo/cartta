use crate::schema::users;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String
}

/*
impl User {
    pub fn create (
        name: String,
        conn: &PgConnection
    ) -> Result<User, AccountError> {
        diesel::insert_into(users::table)
        .values((
            users::name.eq(&name)
        ))
        .get_result::<User>(conn)
        .map_err(AccountError::DatabaseError)
    }

    pub fn update (
        id: i32,
        name: String,
        conn: &PgConnection
    ) -> Result<User, AccountError> {
        diesel::update(users::table.find(id))
        .set((
            users::name.eq(&name)
        ))
        .get_result::<User>(conn)
        .map_err(AccountError::DatabaseError)
    }

    pub fn delete (
        id: i32,
        conn: &PgConnection
    ) -> Result<usize, AccountError> {
        diesel::delete(users::table.find(id))
        .execute(conn)
        .map_err(AccountError::DatabaseError)
    }
}
*/