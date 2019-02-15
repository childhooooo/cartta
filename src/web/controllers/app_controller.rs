use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::request::{self, FlashMessage};
use rocket::response::{Redirect, Flash};

use crate::web::app::DbConn;
use crate::contexts::accounts::{*, user::User};

#[get("/")]
pub fn index_by(user: User) -> Redirect {
    Redirect::to(uri!(editor_by))
}

#[get("/", rank = 2)]
pub fn index(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    let message =
        if let Some(ref msg) = flash {
            msg.msg()
        } else {
            "Helle, world!!"
        };
    context.insert("message", message);
    Template::render("index", &context)
}

#[get("/editor")]
pub fn editor_by(_user: User) -> &'static str {
    "Hello, world!!"
}

#[get("/editor", rank = 2)]
pub fn editor() -> Redirect {
    Redirect::to(uri!(index))
}