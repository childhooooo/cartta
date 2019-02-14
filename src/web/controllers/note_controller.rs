use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Status;
use crate::contexts::accounts::user::User;
use crate::contexts::cms::*;
use crate::contexts::cms::note::*;
use crate::web::app::DbConn;

#[get("/user/<id>?<page>&<per_page>")]
pub fn index(id: i32, page: Option<i64>, per_page: Option<i64>, conn: DbConn) -> Result<Json<Vec<ListNoteWithTag>>, Status> {
    let page = match page {
        Some(page) => page,
        None => 1
    };
    let per_page = match per_page {
        Some(per_page) => per_page,
        None => 30
    };
    let notes = list_owned_notes(&id, page, per_page, &conn).with_tag(&conn)?;
    Ok(Json(notes))
}

/*
#[get("/user/<id>", rank = 2)]
pub fn index(id: i32, conn: DbConn) -> Result<Json<Vec<ListNoteWithTag>>, Status> {
    let notes = list_owned_notes(&id, 1, 30, &conn).with_tag(&conn)?;
    Ok(Json(notes))
}
*/

#[get("/user/<id>/search?<query>&<tag>&<page>&<per_page>")] //Optionにできない？
pub fn search(id: i32, query: Option<String>, tag: Option<String>, page: Option<i64>, per_page: Option<i64>, conn: DbConn) -> Result<Json<Vec<ListNoteWithTag>>, Status> {
    let page = match page {
        Some(page) => page,
        None => 1
    };
    let per_page = match per_page {
        Some(per_page) => per_page,
        None => 30
    };
    let notes =
        if let Some(tag) = tag {
            let tag_ids: Vec<i32> = tag.split(',').map(|t| t.parse().unwrap()).collect();
            if let Some(query) = query {
                search_owned_notes_by_all(&id, query, tag_ids, page, per_page, &conn).with_tag(&conn)?
            } else {
                search_owned_notes_by_tag(&id, tag_ids, page, per_page, &conn).with_tag(&conn)?
            }
        } else {
            if let Some(query) = query {
                search_owned_notes_by_query(&id, query, page, per_page, &conn).with_tag(&conn)?
            } else {
                list_owned_notes(&id, page, per_page, &conn).with_tag(&conn)?
            }
        };
    /*
    let notes =
        if(query.is_none() && tags.is_empty()) {
            list_owned_notes(id, page, per_page, conn)?;
        } else if(query.is_none()) {
            search_owned_notes_by_tag(id, tag_ids, page, per_page, conn)?;
        } else if(tags.is_empty()) {
            search_owned_notes_by_query(id, query.unwrap(), page, per_page, conn)?;
        } else {
            search_owned_notes_by_all(id, query.unwrap(), tag_ids, page, per_page, conn)?;
        }
    */
    Ok(Json(notes))
}

/*
#[get("/user/<id>/search?<query>&<tags>")]
pub fn search(id: i32, query: String, tag_ids: Vec<i32>, page: i64, per_page: i64, conn: DbConn) -> Result<Json<Vec<ListNoteWithTag>>, Status> {
    let notes = search_owned_notes(&id, 1, 30, &conn).with_tag(&conn)?;
    Ok(Json(notes))
}
*/

#[post("/", format = "json", data = "<newnote_json>")]
pub fn create_by(newnote_json: Json<NewNote>, user: User, conn: DbConn) -> Result<Json<Note>, Status> {
    let new = newnote_json.into_inner();
    let note = create_note(user.id, new, &conn)?;
    Ok(Json(note))
}

#[post("/", rank = 2)]
pub fn create() -> Status {
    Status::Unauthorized
}

#[get("/<id>")]
pub fn get(id: i32, conn: DbConn) -> Result<Json<NoteWithTag>, Status> {
    let note = get_note(&id, &conn).with_tag(&conn)?;
    Ok(Json(note))
}

#[put("/<id>", format = "json", data = "<newnote_json>")]
pub fn update_by(id: i32, newnote_json: Json<NewNote>, user: User, conn: DbConn) -> Result<Json<Note>, Status> {
    let new = newnote_json.into_inner();
    let note = edit_note(id, new, &conn)?;
    Ok(Json(note))
}

#[put("/<id>", rank = 2)]
pub fn update(id: i32) -> Status {
    Status::Unauthorized
}

#[put("/<id>/access?<mode>")]
pub fn chmod_by(id: i32, mode: i32, user: User, conn: DbConn) -> Result<Json<Note>, Status> {
    match Access::from_i32(mode) {
        Some(m) => {
            let note = chmod_note(id, m, &conn)?;
            Ok(Json(note))
        },
        None => Err(Status::BadRequest)
    }
}

#[put("/<id>/access?<mode>", rank = 2)]
pub fn chmod(id: i32, mode: i32) -> Status {
    Status::Unauthorized
}