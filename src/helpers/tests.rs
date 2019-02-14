macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => ({
        let _lock = DB_LOCK.lock();
        let (rocket, db) = crate::web::app::rocket();
        let $client = Client::new(rocket).expect("Rocket client");
        let $conn = db.expect("failed to get database connection for testing");
        delete_all_users(&$conn);
        delete_all_notes(&$conn);
        delete_all_tags(&$conn);

        $block
    })
}