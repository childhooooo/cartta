use chrono::NaiveDateTime;

use crate::schema::notes;
use crate::contexts::accounts::user::User;

#[derive(Identifiable, Queryable, Associations, Serialize, Debug, Clone)]
#[belongs_to(User)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub access: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: Option<i32>
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct ListNote {
    pub id: i32,
    pub title: String,
    pub access: i32,
    pub updated_at: NaiveDateTime,
}

pub enum Access {
    Private = 0,
    Protected = 1,
    Public = 2,
}

impl Access {
    pub fn from_i32(n: i32) -> Option<Access> {
        match n {
            0 => Some(Access::Private),
            1 => Some(Access::Protected),
            2 => Some(Access::Public),
            _ => None,
        }
    }
}

/*
impl Note {
    pub fn create (
        user_id: i32,
        title: String,
        content: String,
        conn: &PgConnection
    ) -> Result<Note, CMSError> {
        diesel::insert_into(notes::table)
        .values((
            notes::user_id.eq(&user_id),
            notes::title.eq(&title),
            notes::content.eq(&content),
            notes::access.eq(Access::Private as i32)
        ))
        .get_result::<Note>(conn)
        .map_err(CMSError::DatabaseError)
    }

    pub fn update (
        id: i32,
        title: String,
        content: String,
        conn: &PgConnection
    ) -> Result<Note, CMSError> {
        diesel::update(notes::table.find(id))
        .set((
            notes::title.eq(&title),
            notes::content.eq(&content)
        ))
        .get_result::<Note>(conn)
        .map_err(CMSError::DatabaseError)
    }

    pub fn chmod (
        id: i32,
        access: Access,
        conn: &PgConnection
    ) -> Result<Note, CMSError> {
        diesel::update(notes::table.find(id))
        .set((
            notes::access.eq(access as i32)
        ))
        .get_result::<Note>(conn)
        .map_err(CMSError::DatabaseError)
    }

    pub fn delete (
        id: i32,
        conn: &PgConnection
    ) -> Result<usize, CMSError> {
        diesel::delete(notes::table.find(id))
        .execute(conn)
        .map_err(CMSError::DatabaseError)
    }
}
*/