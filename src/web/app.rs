use rocket::fairing;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use diesel::PgConnection;
use super::controllers::*;
use super::views::note_view::*;
use super::fairings::cors::CORS;

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
        .mount("/", routes![app_controller::index, app_controller::index_by, app_controller::book, app_controller::note, app_controller::signin, app_controller::signin_by, app_controller::signup, app_controller::signup_by])
        .mount("/api/user", routes![user_controller::preflight, user_controller::create, user_controller::get, user_controller::search, user_controller::delete])
        .mount("/api/session", routes![session_controller::preflight, session_controller::create, session_controller::delete, session_controller::nosession])
        .mount("/api/note", routes![note_controller::preflight, note_controller::preflight_id, note_controller::preflight_book, note_controller::preflight_access, note_controller::index, note_controller::index_by, note_controller::book, note_controller::create, note_controller::create_by, note_controller::get, note_controller::get_by, note_controller::update, note_controller::update_by, note_controller::chmod, note_controller::chmod_by, note_controller::delete, note_controller::delete_by])
        .mount("/api/tag", routes![tag_controller::preflight, tag_controller::preflight_id, tag_controller::index, tag_controller::index_by, tag_controller::index_user, tag_controller::create, tag_controller::create_by, tag_controller::delete, tag_controller::delete_by])
        .mount("/static", StaticFiles::from("static"))
        .register(catchers![app_controller::not_found])
        .attach(Template::custom(|engines| {
            engines.tera.register_filter("markdown", filter_markdown);
            engines.tera.register_filter("description", filter_description);
            engines.tera.register_filter("decode", filter_decode);
            engines.tera.register_filter("sanitize", filter_sanitize);
            engines.tera.register_filter("maintitle", filter_title);
            engines.tera.register_filter("capitalize", filter_capitalize);
        }))
        .attach(CORS());

    let conn = match cfg!(test) {
        true => DbConn::get_one(&rocket),
        false => None,
    };

    (rocket, conn)
}