use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Status;
use crate::contexts::accounts::{*, user::User};
use crate::web::app::DbConn;

#[post("/", format = "json", data = "<account_json>")]
pub fn create(account_json: Json<Account>, conn: DbConn) -> Result<Json<User>, Status> {
    let account_new = account_json.into_inner();
    let user = create_user(account_new, &conn)?;
    Ok(Json(user))
}

#[get("/<id>")]
pub fn get(id: i32, conn: DbConn) -> Result<Json<User>, Status> {
    let user = get_user(&id, &conn)?;
    Ok(Json(user))
}

#[get("/name/<name>")]
pub fn search(name: String, conn: DbConn) -> Result<Json<User>, Status> {
    let user = search_user_by_name(&name, &conn)?;
    Ok(Json(user))
}

#[delete("/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Status {
    match delete_user(id, &conn) {
        Ok(_) => Status::Ok,
        Err(err) => Status::from(err)
    }
}

#[options("/")]
pub fn preflight() -> Status {
    Status::Ok
}