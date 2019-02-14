pub mod note;
pub mod tag;
pub mod note_tag;
pub mod error;

use diesel::PgConnection;
use diesel::prelude::*;
use diesel;
use validator::Validate;

use self::note::*;
use self::tag::Tag;
use self::note_tag::NoteTag;
use self::error::CMSError;
use crate::schema::{notes, tags, note_tags};
use crate::helpers::pagination::*;

#[derive(Debug, Serialize)]
pub struct NoteWithTag {
    pub note: Note,
    pub tags: Vec<Tag>
}
#[derive(Debug, Serialize)]
pub struct ListNoteWithTag {
    pub listnote: ListNote,
    pub tags: Vec<Tag>
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewNote {
    #[validate(length(min = "1", max = "60"))]
    pub title: String,
    pub content: String,
    #[validate(length(max = "10"))]
    pub tag_ids: Vec<i32>
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewTag {
    #[validate(length(min = "1", max = "20"))]
    pub name: String
}

pub fn create_note (
    user_id: i32,
    newnote: NewNote,
    conn: &PgConnection
) -> Result<Note, CMSError> {
    newnote.validate()?;
    let note =
        diesel::insert_into(notes::table)
        .values((
            notes::user_id.eq(&user_id),
            notes::title.eq(&newnote.title),
            notes::content.eq(&newnote.content),
            notes::access.eq(Access::Private as i32)
        ))
        .get_result::<Note>(conn)
        .map_err(CMSError::DatabaseError)?;

    for tag_id in newnote.tag_ids {
        add_tag_to(&note.id, &tag_id, conn)?;
    }

    Ok(note)
}

pub fn get_note (
    note_id: &i32,
    conn: &PgConnection
) -> Result<Note, CMSError> {
    let note =
        notes::table
        .find(note_id)
        .first::<Note>(conn)
        .optional()
        .map_err(CMSError::DatabaseError)?;

    note.ok_or(CMSError::NotFound)
}

pub fn list_notes (
    conn: &PgConnection
) -> Result<Vec<ListNote>, CMSError> {
    notes::table
    .order(notes::updated_at.desc())
    .select((
        notes::id,
        notes::title,
        notes::access,
        notes::updated_at
    ))
    .load::<ListNote>(conn)
    .map_err(CMSError::DatabaseError)
}

pub fn list_owned_notes (
    user_id: &i32,
    page: i64,
    per_page: i64,
    conn: &PgConnection
) -> Result<Vec<ListNote>, CMSError> {
    let (list, _total) =
        notes::table
        .filter(notes::user_id.eq(user_id))
        .order(notes::updated_at.desc())
        .select((
            notes::id,
            notes::title,
            notes::access,
            notes::updated_at
        ))
        .paginate(page)
        .per_page(per_page)
        .load_and_count_pages::<ListNote>(conn)
        .map_err(CMSError::DatabaseError)?;
    Ok(list)
}

pub fn search_owned_notes_by_query (
    user_id: &i32,
    query: String,
    page: i64,
    per_page: i64,
    conn: &PgConnection
) -> Result<Vec<ListNote>, CMSError> {
    let pattern = format!("%{}%", query);
    let (list, _total) =
        notes::table
        .filter(notes::user_id.eq(user_id))
        .or_filter(notes::title.ilike(&pattern))
        .or_filter(notes::content.ilike(&pattern))
        .order(notes::updated_at.desc())
        .select((
            notes::id,
            notes::title,
            notes::access,
            notes::updated_at
        ))
        .paginate(page)
        .per_page(per_page)
        .load_and_count_pages::<ListNote>(conn)
        .map_err(CMSError::DatabaseError)?;
    Ok(list)
}

pub fn search_owned_notes_by_tag (
    user_id: &i32,
    tag_ids: Vec<i32>,
    page: i64,
    per_page: i64,
    conn: &PgConnection
) -> Result<Vec<ListNote>, CMSError> {
    let list =
        note_tags::table
        .inner_join(notes::table.on(notes::id.eq(note_tags::note_id)))
        .filter(note_tags::tag_id.eq_any(tag_ids))
        .order((notes::id.desc(), notes::updated_at.desc()))
        .select((
            note_tags::all_columns,
            (
                notes::id,
                notes::title,
                notes::access,
                notes::updated_at
            )
        ))
        .distinct_on(notes::id)
        .paginate(page)
        .per_page(per_page)
        .load_and_count_pages::<(NoteTag, ListNote)>(conn)
        .map_err(CMSError::DatabaseError)?
        .0
        .into_iter()
        .map(|(_, ln)| ln)
        .collect();
    Ok(list)
}

pub fn search_owned_notes_by_all (
    user_id: &i32,
    query: String,
    tag_ids: Vec<i32>,
    page: i64,
    per_page: i64,
    conn: &PgConnection
) -> Result<Vec<ListNote>, CMSError> {
    let pattern = format!("%{}%", query);
    let list =
        note_tags::table
        .inner_join(notes::table.on(notes::id.eq(note_tags::note_id)))
        .filter(note_tags::tag_id.eq_any(tag_ids))
        .or_filter(notes::title.ilike(&pattern))
        .or_filter(notes::content.ilike(&pattern))
        .order((notes::id.desc(), notes::updated_at.desc()))
        .select((
            note_tags::all_columns,
            (
                notes::id,
                notes::title,
                notes::access,
                notes::updated_at
            )
        ))
        .distinct_on(notes::id)
        .paginate(page)
        .per_page(per_page)
        .load_and_count_pages::<(NoteTag, ListNote)>(conn)
        .map_err(CMSError::DatabaseError)?
        .0
        .into_iter()
        .map(|(_, ln)| ln)
        .collect();
    Ok(list)
}

pub fn update_note (
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

pub fn delete_note (
    id: i32,
    conn: &PgConnection
) -> Result<usize, CMSError> {
    diesel::delete(notes::table.find(id))
    .execute(conn)
    .map_err(CMSError::DatabaseError)
}

pub fn chmod_note (
    id: i32,
    access: note::Access,
    conn: &PgConnection
) -> Result<Note, CMSError> {
    diesel::update(notes::table.find(id))
    .set((
        notes::access.eq(access as i32)
    ))
    .get_result::<Note>(conn)
    .map_err(CMSError::DatabaseError)
}

pub fn edit_note (
    note_id: i32,
    newnote: NewNote,
    conn: &PgConnection
) -> Result<Note, CMSError> {
    newnote.validate()?;
    let note = get_note(&note_id, conn)?;
    let tags_added = list_tagged_tags(&note, conn)?;

    for tag in tags_added {
        remove_tag_from(&note.id, &tag.id, conn)?;
    }

    for tag_id in newnote.tag_ids {
        add_tag_to(&note.id, &tag_id, conn)?;
    }

    update_note(note.id, newnote.title, newnote.content, conn)
}

pub fn create_tag (
    user_id: i32,
    tag: NewTag,
    conn: &PgConnection
) -> Result<Tag, CMSError> {
    tag.validate()?;

    diesel::insert_into(tags::table)
    .values((
        tags::name.eq(&tag.name),
        tags::user_id.eq(&user_id)
    ))
    .get_result::<Tag>(conn)
    .map_err(CMSError::DatabaseError)
}

pub fn get_tag (
    tag_id: &i32,
    conn: &PgConnection
) -> Result<Tag, CMSError> {
    let tag =
        tags::table
        .find(tag_id)
        .first::<Tag>(conn)
        .optional()
        .map_err(CMSError::DatabaseError)?;

    tag.ok_or(CMSError::NotFound)
}

pub fn list_tags (
    conn: &PgConnection
) -> Result<Vec<Tag>, CMSError> {
    tags::table
    .order(tags::name.asc())
    .load::<Tag>(conn)
    .map_err(CMSError::DatabaseError)
}

pub fn list_owned_tags (
    user_id: &i32,
    conn: &PgConnection
) -> Result<Vec<Tag>, CMSError> {
    tags::table
    .filter(tags::user_id.eq(user_id))
    .order(tags::name.asc())
    .load::<Tag>(conn)
    .map_err(CMSError::DatabaseError)
}

pub fn list_tagged_tags (
    note: &Note,
    conn: &PgConnection
) -> Result<Vec<Tag>, CMSError> {
    let tag_ids =
        NoteTag::belonging_to(note)
        .select(note_tags::tag_id);

    tags::table
    .filter(tags::id.eq_any(tag_ids))
    .load::<Tag>(conn)
    .map_err(CMSError::DatabaseError)
}

pub fn delete_tag (
    id: i32,
    conn: &PgConnection
) -> Result<usize, CMSError> {
    diesel::delete(tags::table.find(id))
    .execute(conn)
    .map_err(CMSError::DatabaseError)
}

pub fn add_tag_to (
    note_id: &i32,
    tag_id: &i32,
    conn: &PgConnection
) -> Result<usize, CMSError> {
    let note = get_note(&note_id, conn)?;
    let tag = get_tag(&tag_id, conn)?;
    diesel::insert_into(note_tags::table)
    .values((
        note_tags::note_id.eq(note.id),
        note_tags::tag_id.eq(tag.id)
    ))
    .execute(conn)
    .map_err(CMSError::DatabaseError)
}

pub fn remove_tag_from (
    note_id: &i32,
    tag_id: &i32,
    conn: &PgConnection
) -> Result<usize, CMSError> {
    diesel::delete(
        note_tags::table
        .filter(note_tags::note_id.eq(note_id))
        .filter(note_tags::tag_id.eq(tag_id))
    )
    .execute(conn)
    .map_err(CMSError::DatabaseError)
}

pub trait GroupTag<T> {
    fn group_tag (self, parents: &Vec<T>) -> Vec<Vec<Tag>>;
}

impl GroupTag<ListNote> for Vec<(NoteTag, Tag)> {
    fn group_tag (self, parents: &Vec<ListNote>) -> Vec<Vec<Tag>> {
        use std::collections::HashMap;

        let id_indices: HashMap<_, _> = parents
            .iter()
            .enumerate()
            .map(|(i, n)| (n.id, i))
            .collect();
        let mut result = parents.iter().map(|_| Vec::new()).collect::<Vec<_>>();
        for (notetag, tag) in self {
            result[id_indices[&notetag.note_id]].push(tag);
        }
        result
    }
}

impl GroupTag<Note> for Vec<(NoteTag, Tag)> {
    fn group_tag (self, parents: &Vec<Note>) -> Vec<Vec<Tag>> {
        use std::collections::HashMap;

        let id_indices: HashMap<_, _> = parents
            .iter()
            .enumerate()
            .map(|(i, n)| (n.id(), i))
            .collect();
        let mut result = parents.iter().map(|_| Vec::new()).collect::<Vec<_>>();
        for (notetag, tag) in self {
            result[id_indices[&notetag.note_id]].push(tag);
        }
        result
    }
}

/*
impl GroupTag<ListNote> for Vec<(NoteTag, Tag)> {
    fn group_tag (self, parents: &Vec<ListNote>) -> Vec<Vec<Tag>> {
        use std::collections::HashMap;

        let id_indices: HashMap<_, _> = parents
            .iter()
            .enumerate()
            .map(|(i, n)| (n.id(), i))
            .collect();
        let mut result = parents.iter().map(|_| Vec::new()).collect::<Vec<_>>();
        for (notetag, tag) in self {
            result[id_indices[&notetag.note_id]].push(tag);
        }
        result
    }
}
*/

/*
pub fn group_tag(children: Vec<(NoteTag, Tag)>, parents: &Vec<Note>) -> Vec<Vec<Tag>> {
    use std::collections::HashMap;

    let id_indices: HashMap<_, _> = parents
        .iter()
        .enumerate()
        .map(|(i, u)| (u.id(), i))
        .collect();
    let mut result = parents.iter().map(|_| Vec::new()).collect::<Vec<_>>();
    for (notetag, tag) in children {
        result[id_indices[&notetag.note_id]].push(tag);
    }
    result
}
*/

pub trait WithTag<T> {
    fn with_tag (self, conn: &PgConnection) -> Result<T, CMSError>;
}

impl WithTag<Vec<ListNoteWithTag>> for Result<Vec<ListNote>, CMSError> {
    fn with_tag (
        self,
        conn: &PgConnection
    ) -> Result<Vec<ListNoteWithTag>, CMSError> {
        let listnotes = self?;
        let note_ids: Vec<i32> = 
            listnotes
            .iter()
            .map(|n| n.id) // たぶんOK？
            .collect();

        let listnotes_with_tag =
            note_tags::table
            .inner_join(tags::table.on(tags::id.eq(note_tags::tag_id)))
            .filter(note_tags::note_id.eq_any(note_ids))
            .order(tags::name.asc())
            .load::<(NoteTag, Tag)>(conn)
            .map_err(CMSError::DatabaseError)? // tags_with_note_id
            .group_tag(&listnotes)
            .into_iter()
            .enumerate()
            .map(|(i, t)| ListNoteWithTag { listnote: listnotes[i].clone(), tags: t })
            .collect();

        Ok(listnotes_with_tag)


        /*
        Ok(
            group_tag(tags_with_note_id, &listnotes)
            .into_iter()
            .enumerate()
            .map(|(i, t)| NoteWithTag { listnote: listnotes[i].clone(), tags: t })
            .collect()
        )
        */
    }
}

impl WithTag<NoteWithTag> for Result<Note, CMSError> {
    fn with_tag (
        self,
        conn: &PgConnection
    ) -> Result<NoteWithTag, CMSError> {
        let note = self?;
        let list_tags =
            NoteTag::belonging_to(&note)
            .inner_join(tags::table.on(tags::id.eq(note_tags::tag_id)))
            .order(tags::name.asc())
            .load::<(NoteTag, Tag)>(conn)
            .map_err(CMSError::DatabaseError)?
            .into_iter()
            .map(|(nt, t)| t)
            .collect();

        Ok(NoteWithTag { note: note, tags: list_tags })
    }
}