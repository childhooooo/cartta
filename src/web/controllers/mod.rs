pub mod app_controller;
pub mod user_controller;
pub mod session_controller;
pub mod note_controller;
pub mod tag_controller;

use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request, FlashMessage};

use crate::web::app::DbConn;
use crate::contexts::accounts::{*, user::User};

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let conn = request.guard::<DbConn>()?;
        let user_id =
            request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok());
        let user = match user_id {
            Some(user_id) => get_user(&user_id, &conn).ok(),
            None => None
        };
        user.or_forward(())
    }
}