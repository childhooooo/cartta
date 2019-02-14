use rocket::Response;
use rocket::local::Client;
use rocket::http::{Cookie, ContentType};
use diesel::PgConnection;

use crate::contexts::accounts::*;
use crate::contexts::cms::*;

pub fn delete_all_users(conn: &PgConnection) {
    let users = list_users(conn).unwrap();
    for user in users {
        delete_user(user.id, conn).unwrap();
    }
}

pub fn user_id_cookie(response: &Response) -> Option<Cookie<'static>> {
    let cookie =
        response
        .headers()
        .get("Set-Cookie")
        .filter(|v| v.starts_with("user_id"))
        .nth(0)
        .and_then(|val| Cookie::parse_encoded(val).ok());

    cookie.map(|c| c.into_owned())
}

pub fn login(client: &Client, credential: &str) -> Option<Cookie<'static>> {
    let response =
        client
        .post("/session")
        .header(ContentType::JSON)
        .body(credential)
        .dispatch();

    user_id_cookie(&response)
}

pub fn delete_all_notes(conn: &PgConnection) {
    let notes = list_notes(conn).unwrap();
    for note in notes {
        delete_note(note.id, conn).unwrap();
    }
}

pub fn delete_all_tags(conn: &PgConnection) {
    let tags = list_tags(conn).unwrap();
    for tag in tags {
        delete_tag(tag.id, conn).unwrap();
    }
}