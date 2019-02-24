use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Status;
use crate::contexts::accounts::*;
use crate::contexts::accounts::user::User;
use crate::contexts::cms::*;
use crate::contexts::cms::note::*;
use crate::web::app::DbConn;

#[get("/?<query>&<tag>&<page>&<per_page>")]
pub fn index_by(user: User, query: Option<String>, tag: Option<String>, page: Option<i64>, per_page: Option<i64>, conn: DbConn) -> Result<Json<Vec<ListNoteWithTag>>, Status> {
    let notes = list_some_notes(&user.id, query, tag, page, per_page, false, &conn).with_tag(&conn)?;
    Ok(Json(notes))
}

#[get("/", rank = 2)]
pub fn index() -> Status {
    Status::Unauthorized
}

#[get("/user/<id>?<query>&<tag>&<page>&<per_page>")]
pub fn book(id: i32, query: Option<String>, tag: Option<String>, page: Option<i64>, per_page: Option<i64>, conn: DbConn) -> Result<Json<Vec<ListNoteWithTag>>, Status> {
    let notes = list_some_notes(&id, query, tag, page, per_page, true, &conn).with_tag(&conn)?;
    Ok(Json(notes))
}

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
pub fn get_by(user: User, id: i32, conn: DbConn) -> Result<Json<NoteWithTag>, Status> {
    let note = get_note(&id, &conn).with_tag(&conn)?;
    Ok(Json(note))
}

#[get("/<id>", rank = 2)]
pub fn get(id: i32, conn: DbConn) -> Status {
    Status::Unauthorized
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

#[delete("/<id>")]
pub fn delete_by(id: i32, user: User, conn: DbConn) -> Status {
    match delete_note(id, &conn) {
        Ok(_) => Status::Ok,
        Err(err) => Status::from(err)
    }
}

#[delete("/<id>", rank = 2)]
pub fn delete(id: i32) -> Status {
    Status::Unauthorized
}

#[options("/")]
pub fn preflight() -> Status {
    Status::Ok
}

#[options("/<id>")]
pub fn preflight_id(id: i32) -> Status {
    Status::Ok
}

#[options("/user/<id>")]
pub fn preflight_book(id: i32) -> Status {
    Status::Ok
}

#[options("/<id>/access", rank = 2)]
pub fn preflight_access(id: i32) -> Status {
    Status::Ok
}