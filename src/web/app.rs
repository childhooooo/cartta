use rocket::fairing;
use rocket_contrib::templates::Template;
use diesel::PgConnection;
use super::controllers::*;

embed_migrations!();

#[database("postgres_database")]
pub struct DbConn(PgConnection);

pub struct Settings {
    pub days_cookie: i64
}

pub fn rocket() -> (rocket::Rocket, Option<DbConn>) {
    let rocket =
        rocket::ignite()
        .attach(DbConn::fairing())
        .attach(fairing::AdHoc::on_attach("Database Migrations", |rocket| {
            let conn = DbConn::get_one(&rocket).expect("database connection");
            match embedded_migrations::run(&*conn) {
                Ok(()) => Ok(rocket),
                Err(e) => {
                    error!("Failed to run database migrations: {:?}", e);
                    Err(rocket)
                },
            }
        }))
        .attach(fairing::AdHoc::on_attach("Settings", |rocket| {
            let days_cookie = rocket.config().get_int("days_cookie").unwrap_or(30);
            Ok(rocket.manage(Settings{ days_cookie: days_cookie }))
        }))
        .mount("/", routes![app_controller::index, app_controller::index_by, app_controller::editor, app_controller::editor_by])
        .mount("/user", routes![user_controller::create, user_controller::get, user_controller::delete])
        .mount("/session", routes![session_controller::create, session_controller::update, session_controller::delete, session_controller::nosession])
        .mount("/note", routes![note_controller::index, note_controller::index_by, note_controller::book, note_controller::create, note_controller::create_by, note_controller::get, note_controller::get_by, note_controller::update, note_controller::update_by, note_controller::chmod, note_controller::chmod_by])
        .mount("/tag", routes![tag_controller::index, tag_controller::index_by, tag_controller::create, tag_controller::create_by])
        .attach(Template::fairing());

    let conn = match cfg!(test) {
        true => DbConn::get_one(&rocket),
        false => None,
    };

    (rocket, conn)
}