use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use rocket::response::{Flash, Redirect};
use rocket::http::{Cookie, Cookies, SameSite, Status};

use crate::web::app::DbConn;
use crate::web::app::Settings;
use crate::contexts::accounts::{*, user::User};

const ADAY: i64 = 86400;

#[post("/", format = "json", data = "<account_json>")]
pub fn create(mut cookies: Cookies, account_json: Json<Account>, conn: DbConn, settings: State<Settings>) -> Result<Redirect, Flash<Redirect>> {
    let account = account_json.into_inner();
    match authenticate(&account, &conn) {
        Ok(user) => {
            put_session(&user, &settings.days_cookie, &mut cookies);
            Ok(Redirect::to("/editor"))
        },
        Err(err) => {
            Err(Flash::error(Redirect::to("/"), format!("{}", err)))
        }
    }
}

#[put("/", format = "json", data = "<account_json>")]
pub fn update(mut cookies: Cookies, account_json: Json<Account>, conn: DbConn, settings: State<Settings>) -> Result<Json<User>, Status> {
    let account = account_json.into_inner();
    let user = authenticate(&account, &conn)?;
    put_session(&user, &settings.days_cookie, &mut cookies);
    Ok(Json(user))
}

#[delete("/")]
pub fn delete(mut cookies: Cookies, user: User) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to("/")
}

#[delete("/", rank = 2)]
pub fn nosession() -> Status {
    Status::Unauthorized
}

fn put_session(user: &User, expires: &i64, cookies: &mut Cookies) {
    let mut time = time::get_time();
    time.sec += ADAY * expires;

    let mut cookie = Cookie::new("user_id", user.id.to_string());
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_expires(time::at(time));

    cookies.add_private(cookie);
}