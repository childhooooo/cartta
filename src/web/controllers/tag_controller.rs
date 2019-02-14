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