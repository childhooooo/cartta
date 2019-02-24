use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Status;
use crate::contexts::accounts::user::User;
use crate::contexts::cms::*;
use crate::contexts::cms::tag::Tag;
use crate::web::app::DbConn;

#[get("/")]
pub fn index_by(user: User, conn: DbConn) -> Result<Json<Vec<Tag>>, Status> {
    let tags = list_owned_tags(&user.id, &conn)?;
    Ok(Json(tags))
}

#[get("/", rank = 2)]
pub fn index() -> Status {
    Status::Unauthorized
}

#[post("/", format = "json", data = "<newtag_json>")]
pub fn create_by(newtag_json: Json<NewTag>, user: User, conn: DbConn) -> Result<Json<Tag>, Status> {
    let new = newtag_json.into_inner();
    let tag = create_tag(user.id, new, &conn)?;
    Ok(Json(tag))
}

#[post("/", rank = 2)]
pub fn create() -> Status {
    Status::Unauthorized
}

#[get("/<id>")]
pub fn get_by(id: i32, user: User, conn: DbConn) -> Result<Json<Tag>, Status> {
    let tag = get_tag(&id, &conn)?;
    Ok(Json(tag))
}

#[get("/<id>", rank = 2)]
pub fn get(id: i32) -> Status {
    Status::Unauthorized
}

#[get("/user/<id>")]
pub fn index_user(id: i32, conn: DbConn) -> Result<Json<Vec<Tag>>, Status> {
    let tags = list_owned_tags(&id, &conn)?;
    Ok(Json(tags))
}

#[delete("/<id>")]
pub fn delete_by(id: i32, user: User, conn: DbConn) -> Status {
    match delete_tag(id, &conn) {
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