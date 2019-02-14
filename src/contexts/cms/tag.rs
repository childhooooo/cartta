use diesel::prelude::*;
use diesel::insert_into;

use crate::schema::tags;
use crate::contexts::accounts::user::User;

#[derive(Identifiable, Queryable, Serialize, Debug)]
#[belongs_to(User)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub user_id: Option<i32>
}

/*
impl Tag {
    pub fn create (
        name: String,
        conn: &PgConnection
    ) -> Result<Tag, CMSError> {
        diesel::insert_into(tags::table)
        .values((
            tags::name.eq(&name)
        ))
        .get_result::<Tag>(conn)
        .map_err(CMSError::DatabaseError)
    }

    pub fn delete (
        id: i32,
        conn: &PgConnection
    ) -> Result<usize, CMSError> {
        diesel::delete(tags::table.find(id))
        .execute(conn)
        .map_err(CMSError::DatabaseError)
    }
}
*/