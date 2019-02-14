use chrono::NaiveDateTime;

use crate::schema::note_tags;
use super::note::Note;
use super::tag::Tag;

#[derive(Identifiable, Queryable, Associations, Serialize, Debug)]
#[belongs_to(Note)]
#[belongs_to(Tag)]
pub struct NoteTag {
    pub id: i32,
    pub note_id: i32,
    pub tag_id: i32
}