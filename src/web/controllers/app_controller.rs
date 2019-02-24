use rocket_contrib::templates::Template;
use rocket::http::Status;
use std::collections::HashMap;
use rocket::request::{self, FlashMessage};
use rocket::response::{Redirect, Flash};

use crate::web::app::DbConn;
use crate::contexts::accounts::{*, user::User};
use crate::contexts::cms::{* , note::*};
use crate::web::views::{note_view::*, user_view::*};

#[get("/")]
pub fn index_by(user: User) -> Template {
    let mut context = HashMap::new();
    context.insert("user_id", user.id);
    Template::render("index", &context)
}

#[get("/", rank = 2)]
pub fn index() -> Redirect {
    Redirect::to(uri!(signin))
}

#[get("/<name>", rank = 3)]
pub fn book(name: String, conn: DbConn) -> Result<Template, Status> {
    let user = search_user_by_name(&name, &conn)?;
    let context = BookContext {
        user_id: user.id,
        user_name: user.name
    };
    Ok(Template::render("book", &context))
}

#[get("/<name>/<id>")]
pub fn note(name: String, id: i32, conn: DbConn) -> Result<Template, Status> {
    let note = get_note(&id, &conn).with_tag(&conn)?;
    if(note.note.access == Access::Private as i32) { Err(Status::NotFound)? }
    let context = IdContext {
        user_name: name,
        user_id: note.note.user_id.unwrap(),
        title: note.note.title,
        content: note.note.content,
        updated_at: note.note.updated_at,
        tags: note.tags
    };
    Ok(Template::render("id", &context))
}

#[get("/signin")]
pub fn signin_by(user: User) -> Redirect {
    Redirect::to(uri!(index_by))
}

#[get("/signin", rank = 2)]
pub fn signin(flash: Option<FlashMessage>) -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("signin", context)
}

#[get("/signup")]
pub fn signup_by (user: User) -> Redirect {
    Redirect::to(uri!(index_by))
}

#[get("/signup", rank = 2)]
pub fn signup (flash: Option<FlashMessage>) -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("signup", context)
}
#[catch(404)]
pub fn not_found() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("404", context)
}